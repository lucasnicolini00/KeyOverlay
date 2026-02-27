use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, StreamExt, TryStreamExt};
use serde_json::json;
use tauri::{AppHandle, Emitter};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc as tokio_mpsc;
use tokio_tungstenite::tungstenite::Message;

// ── Types ─────────────────────────────────────────────────────────────────────

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

// ── Global state ──────────────────────────────────────────────────────────────

struct AppState {
    peer_map: PeerMap,
    settings_json: Mutex<String>,
    held_keys: Mutex<HeldKeysState>,
    /// Channel to send key combos to the WS broadcaster
    key_tx: tokio_mpsc::UnboundedSender<String>,
    /// Prevents double-starting the tap
    capture_started: AtomicBool,
    /// macOS: stores CFRunLoopRef; Windows: stores hook thread ID — used to stop capture
    run_loop_ref: AtomicUsize,
}

// ── Key name helpers ─────────────────────────────────────────────────────────

/// Returns true for keys that are pure modifiers (Cmd/Win, Shift, Ctrl, Alt, etc.)
fn is_modifier_name(name: &str) -> bool {
    matches!(name, "⌘" | "Win" | "Shift" | "Ctrl" | "Alt" | "Caps" | "Fn")
}

/// Convert a macOS virtual key code (CGKeyCode) to a display string.
#[cfg(target_os = "macos")]
fn keycode_to_name(code: u16) -> String {
    match code {
        // Letters (QWERTY positions)
        0  => "A",  1  => "S",  2  => "D",  3  => "F",
        4  => "H",  5  => "G",  6  => "Z",  7  => "X",
        8  => "C",  9  => "V",  11 => "B",  12 => "Q",
        13 => "W",  14 => "E",  15 => "R",  16 => "Y",
        17 => "T",  31 => "O",  32 => "U",  34 => "I",
        35 => "P",  37 => "L",  38 => "J",  40 => "K",
        45 => "N",  46 => "M",
        // Numbers
        18 => "1",  19 => "2",  20 => "3",  21 => "4",
        23 => "5",  22 => "6",  26 => "7",  28 => "8",
        25 => "9",  29 => "0",
        // Symbols
        24 => "=",  27 => "-",  30 => "]",  33 => "[",
        39 => "'",  41 => ";",  42 => "\\", 43 => ",",
        44 => "/",  47 => ".",  50 => "`",
        // Special
        36 => "↩",  48 => "Tab",  49 => "Space",
        51 => "⌫",  53 => "Esc",
        // Modifiers
        54 | 55 => "⌘",
        56 | 60 => "Shift",
        57 => "Caps",
        58 | 61 => "Alt",
        59 | 62 => "Ctrl",
        63 => "Fn",
        // F-keys
        96  => "F5",   97  => "F6",   98  => "F7",   99  => "F3",
        100 => "F8",   101 => "F9",   103 => "F11",  109 => "F10",
        111 => "F12",  118 => "F4",   120 => "F2",   122 => "F1",
        // Navigation
        115 => "Home",  116 => "PgUp",  117 => "Del",  119 => "End",
        121 => "PgDn",  123 => "←",     124 => "→",    125 => "↓",
        126 => "↑",
        _ => return format!("#{}", code),
    }
    .to_string()
}

/// Convert a Windows virtual key code (VKEY) to a display string.
#[cfg(target_os = "windows")]
fn vkey_to_name(vk: u32) -> String {
    match vk {
        // Letters A-Z (VK_A = 0x41)
        0x41 => "A", 0x42 => "B", 0x43 => "C", 0x44 => "D",
        0x45 => "E", 0x46 => "F", 0x47 => "G", 0x48 => "H",
        0x49 => "I", 0x4A => "J", 0x4B => "K", 0x4C => "L",
        0x4D => "M", 0x4E => "N", 0x4F => "O", 0x50 => "P",
        0x51 => "Q", 0x52 => "R", 0x53 => "S", 0x54 => "T",
        0x55 => "U", 0x56 => "V", 0x57 => "W", 0x58 => "X",
        0x59 => "Y", 0x5A => "Z",
        // Numbers row
        0x30 => "0", 0x31 => "1", 0x32 => "2", 0x33 => "3",
        0x34 => "4", 0x35 => "5", 0x36 => "6", 0x37 => "7",
        0x38 => "8", 0x39 => "9",
        // Special
        0x0D => "↩",   // VK_RETURN
        0x09 => "Tab",  // VK_TAB
        0x20 => "Space",// VK_SPACE
        0x08 => "⌫",   // VK_BACK
        0x1B => "Esc",  // VK_ESCAPE
        0x2E => "Del",  // VK_DELETE
        // Modifiers
        0xA0 | 0xA1 => "Shift",
        0xA2 | 0xA3 => "Ctrl",
        0xA4 | 0xA5 => "Alt",
        0x5B | 0x5C => "Win",   // VK_LWIN / VK_RWIN
        0x14 => "Caps",          // VK_CAPITAL
        // F-keys
        0x70 => "F1",  0x71 => "F2",  0x72 => "F3",  0x73 => "F4",
        0x74 => "F5",  0x75 => "F6",  0x76 => "F7",  0x77 => "F8",
        0x78 => "F9",  0x79 => "F10", 0x7A => "F11", 0x7B => "F12",
        // Navigation
        0x24 => "Home", 0x23 => "End",
        0x21 => "PgUp", 0x22 => "PgDn",
        0x25 => "←",    0x26 => "↑",
        0x27 => "→",    0x28 => "↓",
        // Symbols (OEM)
        0xBB => "=",  0xBD => "-",  0xDB => "[",  0xDD => "]",
        0xDE => "'", 0xBA => ";",  0xDC => "\\", 0xBC => ",",
        0xBF => "/",  0xBE => ".",  0xC0 => "`",
        _ => return format!("#{:02X}", vk),
    }
    .to_string()
}

// ── Held-key state ──────────────────────────────────────────────────────────────

struct HeldKeysState {
    /// Key display names currently held (e.g. "A", "⌘", "Shift")
    keys: HashSet<String>,
    last_event: Instant,
}

impl Default for HeldKeysState {
    fn default() -> Self {
        Self {
            keys: HashSet::new(),
            last_event: Instant::now(),
        }
    }
}

// Settings snapshot for combo logic (read from JSON each time)
#[derive(Clone, serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct ComboSettings {
    #[serde(default = "default_true")]
    combo_mode: bool,
    #[serde(default)]
    show_modifiers_alone: bool,
    #[serde(default)]
    show_mouse_clicks: bool,
    #[serde(default = "default_true")]
    show_mouse_click_combos: bool,
    #[serde(default)]
    key_filter: String,
    #[serde(default = "default_true")]
    key_filter_enabled: bool,
}

fn default_true() -> bool {
    true
}

// ── WebSocket server ──────────────────────────────────────────────────────────

async fn handle_ws_connection(peer_map: PeerMap, stream: TcpStream, addr: SocketAddr, app: AppHandle) {
    let ws_stream = match tokio_tungstenite::accept_async(stream).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[ws] handshake error {addr}: {e}");
            return;
        }
    };

    let (tx, rx) = unbounded::<Message>();
    peer_map.lock().unwrap().insert(addr, tx);

    let count = peer_map.lock().unwrap().len() as u32;
    let _ = app.emit("overlay-clients-changed", count);

    let (mut outgoing, incoming) = ws_stream.split();

    // Drain incoming messages (overlay doesn't send anything, just consume)
    let read = incoming.try_for_each(|_msg| future::ok(()));

    // Forward messages from channel to the WebSocket sink
    let write = rx.map(Ok).forward(&mut outgoing);

    pin_mut!(read, write);
    future::select(read, write).await;

    peer_map.lock().unwrap().remove(&addr);
    let count = peer_map.lock().unwrap().len() as u32;
    let _ = app.emit("overlay-clients-changed", count);
}

async fn run_ws_server(peer_map: PeerMap, app: AppHandle) {
    let listener = TcpListener::bind("127.0.0.1:9001")
        .await
        .expect("[ws] failed to bind port 9001");
    eprintln!("[ws] WebSocket server listening on ws://127.0.0.1:9001");

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_ws_connection(
            peer_map.clone(),
            stream,
            addr,
            app.clone(),
        ));
    }
}

fn broadcast(peer_map: &PeerMap, msg: String) {
    let map = peer_map.lock().unwrap();
    for tx in map.values() {
        let _ = tx.unbounded_send(Message::Text(msg.clone().into()));
    }
}

// ── HTTP server for overlay (serves overlay/index.html) ──────────────────────

/// Overlay HTML embedded at compile time — no runtime path issues in dev or release.
const OVERLAY_HTML: &str = include_str!("../../overlay/index.html");

fn run_http_server() {
    std::thread::spawn(move || {
        let server = tiny_http::Server::http("127.0.0.1:9002").expect("[http] failed to bind port 9002");
        eprintln!("[http] Overlay server listening on http://127.0.0.1:9002");

        for request in server.incoming_requests() {
            let response = tiny_http::Response::from_string(OVERLAY_HTML)
                .with_header(
                    "Content-Type: text/html; charset=utf-8"
                        .parse::<tiny_http::Header>()
                        .unwrap(),
                );
            let _ = request.respond(response);
        }
    });
}

// ── macOS CGEventTap context + callback (listen-only, Input Monitoring perm) ──

/// Context passed to the CGEventTap callback via the user_info void pointer.
#[cfg(target_os = "macos")]
struct TapContext {
    state: Arc<AppState>,
    app: AppHandle,
}

#[cfg(target_os = "macos")]
unsafe extern "C" fn tap_callback(
    _proxy: *mut std::ffi::c_void,
    event_type: u32,
    event: *mut std::ffi::c_void,
    user_info: *mut std::ffi::c_void,
) -> *mut std::ffi::c_void {
    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {
        fn CGEventGetIntegerValueField(event: *mut std::ffi::c_void, field: i32) -> i64;
        fn CGEventGetFlags(event: *mut std::ffi::c_void) -> u64;
    }

    let ctx: &TapContext = &*(user_info as *const TapContext);
    let state = &ctx.state;

    let mut held = state.held_keys.lock().unwrap();
    held.last_event = Instant::now();

    const FLAG_CMD:   u64 = 0x0010_0000;
    const FLAG_SHIFT: u64 = 0x0002_0000;
    const FLAG_CTRL:  u64 = 0x0004_0000;
    const FLAG_ALT:   u64 = 0x0008_0000;

    match event_type {
        12 => {
            // kCGEventFlagsChanged — update modifier set from current flag word
            let flags = CGEventGetFlags(event);

            // Snapshot what was held BEFORE the change to detect newly pressed modifiers
            let had_cmd   = held.keys.contains("⌘");
            let had_shift = held.keys.contains("Shift");
            let had_ctrl  = held.keys.contains("Ctrl");
            let had_alt   = held.keys.contains("Alt");

            held.keys.retain(|k| !matches!(k.as_str(), "⌘" | "Shift" | "Ctrl" | "Alt"));
            let now_cmd   = flags & FLAG_CMD   != 0;
            let now_shift = flags & FLAG_SHIFT != 0;
            let now_ctrl  = flags & FLAG_CTRL  != 0;
            let now_alt   = flags & FLAG_ALT   != 0;
            if now_cmd   { held.keys.insert("⌘".into()); }
            if now_shift { held.keys.insert("Shift".into()); }
            if now_ctrl  { held.keys.insert("Ctrl".into()); }
            if now_alt   { held.keys.insert("Alt".into()); }

            // Emit newly pressed modifier if show_modifiers_alone is on
            let combo_settings: ComboSettings = {
                let json_str = state.settings_json.lock().unwrap();
                serde_json::from_str(&json_str).unwrap_or_default()
            };
            if combo_settings.show_modifiers_alone {
                let mut newly: Vec<&str> = Vec::new();
                if !had_cmd   && now_cmd   { newly.push("⌘"); }
                if !had_shift && now_shift { newly.push("Shift"); }
                if !had_ctrl  && now_ctrl  { newly.push("Ctrl"); }
                if !had_alt   && now_alt   { newly.push("Alt"); }
                if !newly.is_empty() {
                    let names: Vec<String> = newly.iter().map(|s| s.to_string()).collect();
                    drop(held);
                    for name in names {
                        let _ = state.key_tx.send(name.clone());
                        let _ = ctx.app.emit("key-pressed", name);
                    }
                    return event;
                }
            }
        }
        10 | 11 => {
            // kCGEventKeyDown (10) / kCGEventKeyUp (11)
            let keycode = CGEventGetIntegerValueField(event, 9) as u16; // kCGKeyboardEventKeycode = 9
            let key_name = keycode_to_name(keycode);

            if event_type == 11 {
                // keyUp — remove from held set
                held.keys.remove(&key_name);
            } else {
                // keyDown — add and maybe emit a combo
                held.keys.insert(key_name.clone());

                let combo_settings: ComboSettings = {
                    let json_str = state.settings_json.lock().unwrap();
                    serde_json::from_str(&json_str).unwrap_or_default()
                };

                let modifier_only = is_modifier_name(&key_name);
                if !modifier_only || combo_settings.show_modifiers_alone {
                    // Key filter: if enabled and non-empty, only emit keys in the allow-list
                    let allowed = !combo_settings.key_filter_enabled
                        || key_filter_allows(&combo_settings.key_filter, &key_name);
                    if !allowed {
                        return event;
                    }

                    let combo = if combo_settings.combo_mode {
                        build_combo(&held.keys, &key_name)
                    } else {
                        key_name.clone()
                    };

                    drop(held); // release lock before crossing await-point

                    if !combo.is_empty() && combo != "?" {
                        let _ = state.key_tx.send(combo.clone());
                        let _ = ctx.app.emit("key-pressed", combo);
                    }
                    return event;
                }
            }
        }
        1 | 3 => {
            // kCGEventLeftMouseDown (1) / kCGEventRightMouseDown (3)
            let combo_settings: ComboSettings = {
                let json_str = state.settings_json.lock().unwrap();
                serde_json::from_str(&json_str).unwrap_or_default()
            };

            if combo_settings.show_mouse_clicks || combo_settings.show_mouse_click_combos {
                let btn_name = if event_type == 1 { "LClick" } else { "RClick" };
                let has_modifiers = !held.keys.is_empty();

                let output = if has_modifiers && combo_settings.show_mouse_click_combos {
                    // Modifier held + combos enabled → show "Ctrl+LClick"
                    Some(build_combo(&held.keys, btn_name))
                } else if !has_modifiers && combo_settings.show_mouse_clicks {
                    // No modifier + plain clicks enabled → show "LClick"
                    Some(btn_name.to_string())
                } else {
                    None
                };

                drop(held);

                if let Some(combo) = output {
                    let _ = state.key_tx.send(combo.clone());
                    let _ = ctx.app.emit("key-pressed", combo);
                }
                return event;
            }
        }
        _ => {}
    }

    event // listen-only: always return event unchanged
}

// ── Windows low-level hook callback ─────────────────────────────────────────

#[cfg(target_os = "windows")]
struct HookContext {
    state: Arc<AppState>,
    app: AppHandle,
}

// Thread-local so the callback (which has no user_info parameter) can reach state.
#[cfg(target_os = "windows")]
thread_local! {
    static HOOK_CTX: std::cell::RefCell<Option<*const HookContext>> =
        std::cell::RefCell::new(None);
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn keyboard_hook_proc(
    code: i32,
    wparam: usize,
    lparam: isize,
) -> isize {
    use windows_sys::Win32::UI::WindowsAndMessaging::*;
    if code >= 0 {
        let kb = &*(lparam as *const KBDLLHOOKSTRUCT);
        // WM_KEYDOWN = 0x100, WM_SYSKEYDOWN = 0x104
        let is_down = wparam == 0x100 || wparam == 0x104;
        let is_up   = wparam == 0x101 || wparam == 0x105;
        let vk = kb.vkCode;
        let key_name = vkey_to_name(vk);

        HOOK_CTX.with(|cell| {
            if let Some(ptr) = *cell.borrow() {
                let ctx = &*ptr;
                let state = &ctx.state;
                let mut held = state.held_keys.lock().unwrap();
                held.last_event = Instant::now();

                if is_up {
                    held.keys.remove(&key_name);
                } else if is_down {
                    // For modifier keys, snapshot before inserting
                    let is_mod = is_modifier_name(&key_name);
                    let was_held = held.keys.contains(&key_name);
                    held.keys.insert(key_name.clone());

                    // Suppress key repeat for modifiers
                    if is_mod && was_held {
                        return;
                    }

                    let combo_settings: ComboSettings = {
                        let json_str = state.settings_json.lock().unwrap();
                        serde_json::from_str(&json_str).unwrap_or_default()
                    };

                    if !is_mod || combo_settings.show_modifiers_alone {
                        // Key filter: if enabled and non-empty, only emit keys in the allow-list
                        if combo_settings.key_filter_enabled
                            && !key_filter_allows(&combo_settings.key_filter, &key_name)
                        {
                            return;
                        }

                        let combo = if combo_settings.combo_mode {
                            build_combo(&held.keys, &key_name)
                        } else {
                            key_name.clone()
                        };
                        drop(held);
                        if !combo.is_empty() {
                            let _ = state.key_tx.send(combo.clone());
                            let _ = ctx.app.emit("key-pressed", combo);
                        }
                        return;
                    }
                }
            }
        });
    }
    CallNextHookEx(0, code, wparam, lparam)
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn mouse_hook_proc(
    code: i32,
    wparam: usize,
    lparam: isize,
) -> isize {
    use windows_sys::Win32::UI::WindowsAndMessaging::*;
    if code >= 0 {
        // WM_LBUTTONDOWN = 0x201, WM_RBUTTONDOWN = 0x204
        let is_ldown = wparam == 0x201;
        let is_rdown = wparam == 0x204;
        if is_ldown || is_rdown {
            HOOK_CTX.with(|cell| {
                if let Some(ptr) = *cell.borrow() {
                    let ctx = &*ptr;
                    let state = &ctx.state;
                    let combo_settings: ComboSettings = {
                        let json_str = state.settings_json.lock().unwrap();
                        serde_json::from_str(&json_str).unwrap_or_default()
                    };
                    if combo_settings.show_mouse_clicks || combo_settings.show_mouse_click_combos {
                        let btn = if is_ldown { "LClick" } else { "RClick" };
                        let held = state.held_keys.lock().unwrap();
                        let has_modifiers = !held.keys.is_empty();

                        let output = if has_modifiers && combo_settings.show_mouse_click_combos {
                            Some(build_combo(&held.keys, btn))
                        } else if !has_modifiers && combo_settings.show_mouse_clicks {
                            Some(btn.to_string())
                        } else {
                            None
                        };
                        drop(held);
                        if let Some(combo) = output {
                            let _ = state.key_tx.send(combo.clone());
                            let _ = ctx.app.emit("key-pressed", combo);
                        }
                    }
                }
            });
        }
    }
    CallNextHookEx(0, code, wparam, lparam)
}

// ── Permission check (no prompt) ────────────────────────────────────────────

/// Silent preflight: can THIS process create a CGEventTap?
/// Uses CGPreflightListenEventAccess() which checks Input Monitoring permission
/// for the actual binary — unaffected by parent-process (VS Code) inheritance.
fn ax_is_trusted() -> bool {
    #[cfg(target_os = "macos")]
    {
        #[link(name = "CoreGraphics", kind = "framework")]
        extern "C" {
            fn CGPreflightListenEventAccess() -> u8;
        }
        unsafe { CGPreflightListenEventAccess() != 0 }
    }
    #[cfg(target_os = "windows")]
    {
        true // Windows low-level hooks require no special permission
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        true
    }
}

// ── Key capture thread ────────────────────────────────────────────────────────

/// Spawns a CGEventTap in listen-only mode (requires Input Monitoring permission).
/// MUST only be called after ax_is_trusted() returns true.
fn start_key_listener(state: Arc<AppState>, app: AppHandle) {
    std::thread::spawn(move || {
        // Brief delay so the Tauri window is fully up
        std::thread::sleep(Duration::from_millis(300));

        // Stale-key cleanup thread
        {
            let sc = state.clone();
            std::thread::spawn(move || loop {
                std::thread::sleep(Duration::from_secs(5));
                let mut held = sc.held_keys.lock().unwrap();
                if held.last_event.elapsed() > Duration::from_secs(5) {
                    held.keys.clear();
                }
            });
        }

        #[cfg(target_os = "macos")]
        {
            use std::ffi::c_void;

            #[link(name = "CoreGraphics", kind = "framework")]
            extern "C" {
                fn CGEventTapCreate(
                    tap:                u32,
                    place:              u32,
                    options:            u32,
                    events_of_interest: u64,
                    callback: unsafe extern "C" fn(*mut c_void, u32, *mut c_void, *mut c_void) -> *mut c_void,
                    user_info:          *mut c_void,
                ) -> *mut c_void;
                fn CGEventTapEnable(tap: *mut c_void, enable: u8);
            }

            #[link(name = "CoreFoundation", kind = "framework")]
            extern "C" {
                fn CFMachPortCreateRunLoopSource(
                    alloc: *const c_void, port: *mut c_void, order: isize,
                ) -> *mut c_void;
                fn CFRunLoopGetCurrent() -> *mut c_void;
                fn CFRunLoopAddSource(rl: *mut c_void, source: *mut c_void, mode: *const c_void);
                fn CFRunLoopRun();
                static kCFRunLoopCommonModes: *const c_void;
            }

            // Leak the context — this thread runs for the app's lifetime
            let ctx = Box::leak(Box::new(TapContext { state, app: app.clone() }));

            let tap = unsafe {
                CGEventTapCreate(
                    0,   // kCGHIDEventTap
                    0,   // kCGHeadInsertEventTap
                    0x1, // kCGEventTapOptionListenOnly  ← Input Monitoring only, no Accessibility
                    (1 << 1) | (1 << 3) | (1 << 10) | (1 << 11) | (1 << 12), // LMouseDown | RMouseDown | keyDown | keyUp | flagsChanged
                    tap_callback,
                    ctx as *mut TapContext as *mut c_void,
                )
            };

            if tap.is_null() {
                eprintln!("[KeyOverlay] CGEventTapCreate returned null — grant Input Monitoring in System Settings → Privacy");
                let _ = app.emit("key-capture-error", "no_input_monitoring");
                return;
            }

            unsafe {
                let source = CFMachPortCreateRunLoopSource(std::ptr::null(), tap, 0);
                let rl = CFRunLoopGetCurrent();
                CFRunLoopAddSource(rl, source, kCFRunLoopCommonModes);
                CGEventTapEnable(tap, 1);
                // Store run loop pointer so stop_key_capture can stop it
                ctx.state.run_loop_ref.store(rl as usize, Ordering::SeqCst);
                eprintln!("[KeyOverlay] ⌨  CGEventTap active (listen-only, Input Monitoring)");
                CFRunLoopRun(); // blocks until CFRunLoopStop() is called
                // Cleanup after stop
                ctx.state.run_loop_ref.store(0, Ordering::SeqCst);
                ctx.state.capture_started.store(false, Ordering::SeqCst);
                ctx.state.held_keys.lock().unwrap().keys.clear();
                eprintln!("[KeyOverlay] ⌨  CGEventTap stopped");
            }
        }

        #[cfg(target_os = "windows")]
        {
            use windows_sys::Win32::{
                Foundation::HWND,
                System::Threading::GetCurrentThreadId,
                UI::WindowsAndMessaging::{
                    CallNextHookEx, DispatchMessageW, GetMessageW,
                    SetWindowsHookExW, UnhookWindowsHookEx,
                    MSG, WH_KEYBOARD_LL, WH_MOUSE_LL,
                },
            };

            // Leak context — lives until the message loop exits
            let ctx = Box::leak(Box::new(HookContext { state, app: app.clone() }));

            unsafe {
                HOOK_CTX.with(|cell| {
                    *cell.borrow_mut() = Some(ctx as *const HookContext);
                });

                let hk_kb = SetWindowsHookExW(
                    WH_KEYBOARD_LL,
                    Some(keyboard_hook_proc),
                    0,
                    0,
                );
                let hk_ms = SetWindowsHookExW(
                    WH_MOUSE_LL,
                    Some(mouse_hook_proc),
                    0,
                    0,
                );

                if hk_kb == 0 || hk_ms == 0 {
                    eprintln!("[KeyOverlay] SetWindowsHookExW failed");
                    let _ = app.emit("key-capture-error", "hook_failed");
                    return;
                }

                // Store thread ID so stop_key_capture can post WM_QUIT
                let tid = GetCurrentThreadId();
                ctx.state.run_loop_ref.store(tid as usize, Ordering::SeqCst);
                eprintln!("[KeyOverlay] ⌨  Windows hooks active (keyboard + mouse)");

                // Win32 message loop — blocks until WM_QUIT is posted
                let mut msg = std::mem::zeroed::<MSG>();
                while GetMessageW(&mut msg, 0 as HWND, 0, 0) > 0 {
                    DispatchMessageW(&msg);
                }

                UnhookWindowsHookEx(hk_kb);
                UnhookWindowsHookEx(hk_ms);
                HOOK_CTX.with(|cell| *cell.borrow_mut() = None);
                ctx.state.run_loop_ref.store(0, Ordering::SeqCst);
                ctx.state.capture_started.store(false, Ordering::SeqCst);
                ctx.state.held_keys.lock().unwrap().keys.clear();
                eprintln!("[KeyOverlay] ⌨  Windows hooks stopped");
            }
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        eprintln!("[KeyOverlay] Key capture not yet supported on this platform");
    });
}

/// Returns true if the key should be shown given the filter string.
/// Filter is a comma/space-separated list of key names (case-insensitive).
/// An empty filter means "show everything".
fn key_filter_allows(filter: &str, key_name: &str) -> bool {
    let filter = filter.trim();
    if filter.is_empty() {
        return true;
    }
    let key_upper = key_name.to_uppercase();
    filter
        .split(|c: char| c == ',' || c == ' ')
        .map(|s| s.trim().to_uppercase())
        .filter(|s| !s.is_empty())
        .any(|allowed| allowed == key_upper)
}

/// Build a combo string: ⌘+Ctrl+Alt+Shift+<key>
fn build_combo(held: &HashSet<String>, trigger: &str) -> String {
    let has_meta  = held.contains("⌘");
    let has_ctrl  = held.contains("Ctrl");
    let has_alt   = held.contains("Alt");
    let has_shift = held.contains("Shift");
    let is_mod    = is_modifier_name(trigger);

    let mut parts: Vec<&str> = Vec::new();
    if has_meta  && trigger != "⌘"     { parts.push("⌘");     }
    if has_ctrl  && trigger != "Ctrl"  { parts.push("Ctrl");  }
    if has_alt   && trigger != "Alt"   { parts.push("Alt");   }
    if has_shift && trigger != "Shift" { parts.push("Shift"); }

    let mut result = parts.join("+");
    if !result.is_empty() && !is_mod {
        result.push('+');
    }
    result.push_str(trigger);
    result
}

// ── Tauri commands ────────────────────────────────────────────────────────────

#[tauri::command]
fn broadcast_settings(
    settings: String,
    state: tauri::State<Arc<AppState>>,
) {
    // Update stored settings JSON
    *state.settings_json.lock().unwrap() = settings.clone();

    // Parse to broadcast to overlay
    if let Ok(data) = serde_json::from_str::<serde_json::Value>(&settings) {
        let msg = json!({ "type": "settings", "data": data }).to_string();
        broadcast(&state.peer_map, msg);
    }
}

#[tauri::command]
fn check_accessibility() -> bool {
    #[cfg(target_os = "macos")]
    {
        // Prompts for Input Monitoring permission on macOS.
        #[link(name = "CoreGraphics", kind = "framework")]
        extern "C" {
            fn CGRequestListenEventAccess() -> u8;
        }
        unsafe { CGRequestListenEventAccess() != 0 }
    }
    #[cfg(not(target_os = "macos"))]
    {
        true // Windows and other platforms need no special permission
    }
}

#[tauri::command]
fn start_key_capture(
    app: AppHandle,
    state: tauri::State<Arc<AppState>>,
) -> Result<String, String> {
    if state.capture_started.load(Ordering::SeqCst) {
        return Ok("already_running".to_string());
    }
    if !ax_is_trusted() {
        return Err("accessibility_denied".to_string());
    }
    state.capture_started.store(true, Ordering::SeqCst);
    start_key_listener(Arc::clone(state.inner()), app);
    Ok("started".to_string())
}

#[tauri::command]
fn stop_key_capture(state: tauri::State<Arc<AppState>>) -> Result<String, String> {
    let handle = state.run_loop_ref.load(Ordering::SeqCst);
    if handle != 0 {
        #[cfg(target_os = "macos")]
        unsafe {
            #[link(name = "CoreFoundation", kind = "framework")]
            extern "C" {
                fn CFRunLoopStop(rl: *mut std::ffi::c_void);
            }
            CFRunLoopStop(handle as *mut std::ffi::c_void);
        }
        #[cfg(target_os = "windows")]
        unsafe {
            use windows_sys::Win32::UI::WindowsAndMessaging::{PostThreadMessageW, WM_QUIT};
            PostThreadMessageW(handle as u32, WM_QUIT, 0, 0);
        }
    } else {
        // Hook/loop not started yet (edge case) — just reset the flag
        state.capture_started.store(false, Ordering::SeqCst);
        state.held_keys.lock().unwrap().keys.clear();
    }
    Ok("stopped".to_string())
}

// ── App entry point ───────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let peer_map: PeerMap = Arc::new(Mutex::new(HashMap::new()));

    // Create the broadcaster channel here so key_tx can live in AppState
    let (key_tx, mut key_rx) = tokio_mpsc::unbounded_channel::<String>();

    let app_state = Arc::new(AppState {
        peer_map: peer_map.clone(),
        settings_json: Mutex::new("{}".to_string()),
        held_keys: Mutex::new(HeldKeysState::default()),
        key_tx,
        capture_started: AtomicBool::new(false),
        run_loop_ref: AtomicUsize::new(0),
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .manage(app_state.clone())
        .invoke_handler(tauri::generate_handler![
            broadcast_settings,
            check_accessibility,
            start_key_capture,
            stop_key_capture
        ])
        .setup(move |app| {
            let handle = app.handle().clone();

            // ── HTTP server for overlay page ───────────────────────────────
            run_http_server();

            // ── WebSocket server ───────────────────────────────────────────
            let ws_peer_map = peer_map.clone();
            let ws_handle = handle.clone();
            std::thread::spawn(move || {
                tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(run_ws_server(ws_peer_map, ws_handle));
            });

            // ── WS broadcaster (forwards key combos to all overlay clients) ─
            let broadcast_peer_map = peer_map.clone();
            std::thread::spawn(move || {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async move {
                        while let Some(combo) = key_rx.recv().await {
                            let msg = json!({ "type": "keypress", "combo": combo }).to_string();
                            broadcast(&broadcast_peer_map, msg);
                        }
                    });
            });

            // ── Emit initial accessibility status ─────────────────────────
            let trusted = ax_is_trusted();
            let _ = handle.emit("accessibility-status", trusted);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
