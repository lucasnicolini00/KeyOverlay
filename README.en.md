# KeyOverlay

A lightweight keystroke & mouse-click overlay for streamers and content creators.  
**Works out of the box on Windows — no permissions, no setup, just run and stream.**

Designed to be used as an **OBS Browser Source** at `http://localhost:9002`.

![KeyOverlay screenshot](docs/screenshot.png)

---

## Quick Start (Windows)

1. **Download** the latest `.exe` installer from [Releases](../../releases)
2. Run the installer — Windows may show a SmartScreen prompt, click **"More info → Run anyway"**
3. Launch **KeyOverlay**
4. Click **Start** to begin capturing keystrokes
5. In OBS, add a **Browser Source** → URL: `http://localhost:9002`
6. Done ✅

> No administrator rights, no drivers, no extra software needed.

---

## OBS Browser Source Settings

| Setting                                   | Value                   |
| ----------------------------------------- | ----------------------- |
| URL                                       | `http://localhost:9002` |
| Width                                     | `1920`                  |
| Height                                    | `120`                   |
| Shutdown source when not visible          | **OFF**                 |
| Refresh browser when scene becomes active | **ON**                  |
| Custom CSS                                | _(leave empty)_         |

---

## Features

- **Real-time keystroke display** — animated badges for every key press
- **Key filter** — choose exactly which keys appear on stream; everything else is silently ignored
- **Mouse click support** — LClick / RClick badges, with optional modifier combos (`Ctrl+LClick`)
- **Combo mode** — shows held modifiers with each key (`Ctrl+Shift+K`)
- **Modifier-alone mode** — optionally show standalone Ctrl / Alt / Shift / Win presses
- **Instant settings sync** — changes appear in OBS immediately, no reload needed
- **Multiple presets** — Minimal, Gaming, Retro, Neon
- **Persistent settings** — remembered between sessions

---

## Settings Reference

| Setting                  | Description                                                          |
| ------------------------ | -------------------------------------------------------------------- |
| **Key filter**           | Comma-separated allowlist of keys — only those keys appear on stream |
| **Combo mode**           | Prefix keys with held modifiers (`Ctrl+Shift+K`)                     |
| **Show modifiers alone** | Show a badge when only Ctrl / Alt / Shift is pressed                 |
| **Show mouse clicks**    | Show LClick / RClick badges                                          |
| **Show click combos**    | Include modifiers on click badges (`Ctrl+LClick`)                    |
| **Layout**               | `horizontal` or `vertical` badge stack                               |
| **Animation style**      | `pop`, `fade`, or `slide`                                            |
| **Max visible keys**     | How many badges are shown at once                                    |
| **Key display duration** | How long each badge stays on screen (ms)                             |

---

## Presets

| Preset    | Animation | Key combos | Click combos |
| --------- | --------- | ---------- | ------------ |
| ○ Minimal | Pop       | ✅         | ✅           |
| 🎮 Gaming | Pop       | ✅         | ✅           |
| 👾 Retro  | Fade      | ✅         | ❌           |
| ✨ Neon   | Pop       | ✅         | ✅           |

---

## Key Filter — Show Only What You Want

KeyOverlay lets you choose **exactly which keys appear on your stream**. Any key not in your list is silently ignored — it never reaches OBS.

This is perfect for:

- 🎮 **Games** — show only movement and ability keys (`W, A, S, D, Q, E, R, F`)
- 🕹️ **MOBAs / shooters** — filter to ability numbers (`1, 2, 3, 4, 5, 6`)
- 📺 **Privacy** — prevent passwords or accidental keypresses from showing up on stream

**How to use it:**

1. In the **OBS Browser Source** section of the app, enable **Key filter**
2. Type the keys you want to show, separated by commas
3. Changes apply instantly — no OBS restart needed

**Examples:**

| Use case         | Filter                 |
| ---------------- | ---------------------- |
| WASD + abilities | `W,A,S,D,Q,E,R,F`      |
| Ability numbers  | `1,2,3,4,5,6`          |
| Classic movement | `W,A,S,D,Space`        |
| Show everything  | _(disable the filter)_ |

> The filter is case-insensitive. `q` and `Q` are treated the same.

---

## Windows SmartScreen Warning

When you download KeyOverlay, Windows may show this message:

> _"Windows protected your PC — Microsoft Defender SmartScreen prevented an unrecognized app from starting."_

This is **completely normal**. It does not mean the app is malicious.

**Why does this happen?**  
Windows requires a **code signing certificate** to automatically trust a downloaded executable. These certificates cost $200–$500 USD per year — not a reasonable expense for a free hobby project. KeyOverlay ships unsigned, just like most indie and open-source tools do.

**What to do:**  
Click **"More info" → "Run anyway"** and the app will open normally. You only need to do this once.

If you'd rather verify it yourself, the full source code is available in this repository.

---

## Building from Source

Requirements: [Node.js](https://nodejs.org/) 18+, [Rust](https://rustup.rs/) 1.77+, [Tauri CLI](https://tauri.app/start/prerequisites/)

```bash
npm install
npm run tauri dev     # development (hot-reload)
npm run tauri build   # production installer → src-tauri/target/release/bundle/
```

---

## Platform Notes

| Platform          | Status          | Notes                                |
| ----------------- | --------------- | ------------------------------------ |
| **Windows 10/11** | ✅ Full support | No permissions needed                |
| macOS 12+         | ✅ Supported    | Requires Input Monitoring permission |

---

## License

MIT
