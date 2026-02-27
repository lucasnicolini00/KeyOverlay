import { useState, useEffect, useCallback } from "react";
import { load } from "@tauri-apps/plugin-store";
import { invoke } from "@tauri-apps/api/core";
import { AppSettings, DEFAULT_SETTINGS } from "../types/settings";

const STORE_KEY = "settings";
const STORE_OPTIONS = {
  defaults: { [STORE_KEY]: DEFAULT_SETTINGS as unknown },
  autoSave: true as const,
};

export function useSettings() {
  const [settings, setSettingsState] = useState<AppSettings>(DEFAULT_SETTINGS);
  const [loaded, setLoaded] = useState(false);

  // Load persisted settings on mount
  useEffect(() => {
    (async () => {
      try {
        const store = await load("settings.json", STORE_OPTIONS);
        const saved = await store.get<AppSettings>(STORE_KEY);
        if (saved) {
          const merged = { ...DEFAULT_SETTINGS, ...saved };
          // If keyFilter was saved as empty (old default), restore the new default
          if (!merged.keyFilter) {
            merged.keyFilter = DEFAULT_SETTINGS.keyFilter;
            merged.keyFilterEnabled = DEFAULT_SETTINGS.keyFilterEnabled;
          }
          // Force showMouseClickCombos to true if it was saved as false (old default)
          if (!merged.showMouseClickCombos) {
            merged.showMouseClickCombos = true;
          }
          // Force showMouseClicks to false (new default)
          merged.showMouseClicks = false;
          setSettingsState(merged);
          // Broadcast to Rust so key filter and other settings take effect immediately
          await invoke("broadcast_settings", {
            settings: JSON.stringify(merged),
          });
        }
      } catch (e) {
        console.error("Failed to load settings:", e);
      } finally {
        setLoaded(true);
      }
    })();
  }, []);

  const updateSettings = useCallback(
    async (patch: Partial<AppSettings>) => {
      const next = { ...settings, ...patch };
      setSettingsState(next);
      try {
        // Persist to store
        const store = await load("settings.json", STORE_OPTIONS);
        await store.set(STORE_KEY, next);
        // Broadcast to overlay via Rust WebSocket server
        await invoke("broadcast_settings", {
          settings: JSON.stringify(next),
        });
      } catch (e) {
        console.error("Failed to save/broadcast settings:", e);
      }
    },
    [settings],
  );

  return { settings, updateSettings, loaded };
}
