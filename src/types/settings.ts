export type AnimationStyle = "fade" | "slide" | "pop";
export type LayoutDirection = "horizontal" | "vertical";
export type FontFamily =
  | "Inter"
  | "monospace"
  | "JetBrains Mono"
  | "Press Start 2P"
  | "system-ui";

export interface AppSettings {
  // Typography
  fontFamily: FontFamily;
  fontSize: number; // px

  // Colors
  textColor: string; // hex
  borderColor: string; // hex
  backgroundColor: string; // rgba

  // Border
  borderWidth: number; // px
  borderRadius: number; // px

  // Effects
  backgroundBlur: number; // px
  textShadow: boolean;
  textShadowColor: string;

  // Behavior
  keyDisplayDuration: number; // ms
  showModifiersAlone: boolean;
  showMouseClicks: boolean;
  showMouseClickCombos: boolean;
  comboMode: boolean;
  keyFilter: string;
  keyFilterEnabled: boolean; // comma/space-separated list of keys to show; empty = show all

  // Layout
  layout: LayoutDirection;
  animationStyle: AnimationStyle;
  maxVisibleKeys: number;

  // Preset name (null = custom)
  preset: string | null;
}

export type PresetName = "minimal" | "gaming" | "retro" | "neon";

export const PRESETS: Record<PresetName, Omit<AppSettings, "preset">> = {
  minimal: {
    fontFamily: "Inter",
    fontSize: 28,
    textColor: "#ffffff",
    borderColor: "#ffffff",
    backgroundColor: "rgba(0,0,0,0.35)",
    borderWidth: 1,
    borderRadius: 8,
    backgroundBlur: 4,
    textShadow: false,
    textShadowColor: "#000000",
    keyDisplayDuration: 1200,
    showModifiersAlone: false,
    showMouseClicks: false,
    showMouseClickCombos: true,
    comboMode: true,
    keyFilter: "Q,W,E,R,1,2,3,4,5,6",
    keyFilterEnabled: true,
    layout: "horizontal",
    animationStyle: "pop",
    maxVisibleKeys: 5,
  },
  gaming: {
    fontFamily: "Inter",
    fontSize: 32,
    textColor: "#00ff88",
    borderColor: "#00ff88",
    backgroundColor: "rgba(0,0,0,0.6)",
    borderWidth: 2,
    borderRadius: 6,
    backgroundBlur: 0,
    textShadow: true,
    textShadowColor: "#00ff88",
    keyDisplayDuration: 900,
    showModifiersAlone: false,
    showMouseClicks: false,
    showMouseClickCombos: true,
    comboMode: true,
    keyFilter: "Q,W,E,R,1,2,3,4,5,6",
    keyFilterEnabled: true,
    layout: "horizontal",
    animationStyle: "pop",
    maxVisibleKeys: 6,
  },
  retro: {
    fontFamily: "Press Start 2P",
    fontSize: 16,
    textColor: "#ffff00",
    borderColor: "#ffff00",
    backgroundColor: "rgba(0,0,0,0.85)",
    borderWidth: 3,
    borderRadius: 0,
    backgroundBlur: 0,
    textShadow: false,
    textShadowColor: "#000000",
    keyDisplayDuration: 1500,
    showModifiersAlone: false,
    showMouseClicks: false,
    showMouseClickCombos: true,
    comboMode: true,
    keyFilter: "Q,W,E,R,1,2,3,4,5,6",
    keyFilterEnabled: true,
    layout: "horizontal",
    animationStyle: "fade",
    maxVisibleKeys: 4,
  },
  neon: {
    fontFamily: "Inter",
    fontSize: 30,
    textColor: "#ff00ff",
    borderColor: "#ff00ff",
    backgroundColor: "rgba(20,0,40,0.5)",
    borderWidth: 2,
    borderRadius: 12,
    backgroundBlur: 8,
    textShadow: true,
    textShadowColor: "#ff00ff",
    keyDisplayDuration: 1000,
    showModifiersAlone: false,
    showMouseClicks: false,
    showMouseClickCombos: true,
    comboMode: true,
    keyFilter: "Q,W,E,R,1,2,3,4,5,6",
    keyFilterEnabled: true,
    layout: "horizontal",
    animationStyle: "pop",
    maxVisibleKeys: 5,
  },
};

export const DEFAULT_SETTINGS: AppSettings = {
  ...PRESETS.minimal,
  fontFamily: "monospace",
  preset: "minimal",
  showMouseClickCombos: true,
};
