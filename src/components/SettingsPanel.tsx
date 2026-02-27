import {
  AppSettings,
  DEFAULT_SETTINGS,
  PRESETS,
  PresetName,
  FontFamily,
  AnimationStyle,
  LayoutDirection,
} from "../types/settings";
import { KeyPreview } from "./KeyPreview";
import { useLang } from "../i18n/LangContext";
import "../styles/settings-panel.css";

interface Props {
  settings: AppSettings;
  onUpdate: (patch: Partial<AppSettings>) => void;
}

const FONTS: FontFamily[] = [
  "monospace",
  "Inter",
  "system-ui",
  "JetBrains Mono",
  "Press Start 2P",
];
const PRESET_NAMES: PresetName[] = ["minimal", "gaming", "retro", "neon"];
const PRESET_ICONS: Record<PresetName, string> = {
  minimal: "○",
  gaming: "🎮",
  retro: "👾",
  neon: "✨",
};

export function SettingsPanel({ settings, onUpdate }: Props) {
  const { t } = useLang();

  const applyPreset = (name: PresetName) => {
    onUpdate({ ...PRESETS[name], preset: name });
  };

  const resetToDefaults = () => {
    onUpdate({ ...DEFAULT_SETTINGS });
  };

  const s = settings;

  return (
    <div className="settings-panel">
      {/* PRESETS */}
      <section className="section">
        <h2 className="section-title">{t.sectionPresets}</h2>
        <div className="preset-row">
          {PRESET_NAMES.map((name) => (
            <button
              key={name}
              className={`preset-btn ${s.preset === name ? "active" : ""}`}
              onClick={() => applyPreset(name)}
            >
              <span className="preset-icon">{PRESET_ICONS[name]}</span>
              <span className="preset-label">{name}</span>
            </button>
          ))}
        </div>
      </section>

      {/* PREVIEW */}
      <section className="section">
        <div className="section-title-row">
          <h2 className="section-title">{t.sectionPreview}</h2>
          <button
            className="reset-btn"
            onClick={resetToDefaults}
            title={t.resetTitle}
          >
            {t.resetBtn}
          </button>
        </div>
        <KeyPreview settings={s} />
      </section>

      {/* TYPOGRAPHY */}
      <section className="section">
        <h2 className="section-title">{t.sectionTypography}</h2>
        <div className="row">
          <label>{t.font}</label>
          <select
            value={s.fontFamily}
            onChange={(e) =>
              onUpdate({
                fontFamily: e.target.value as FontFamily,
                preset: null,
              })
            }
          >
            {FONTS.map((f) => (
              <option key={f} value={f}>
                {f}
              </option>
            ))}
          </select>
        </div>
        <div className="row">
          <label>{t.fontSize(s.fontSize)}</label>
          <input
            type="range"
            min={12}
            max={72}
            value={s.fontSize}
            onChange={(e) =>
              onUpdate({ fontSize: Number(e.target.value), preset: null })
            }
          />
        </div>
      </section>

      {/* COLORS */}
      <section className="section">
        <h2 className="section-title">{t.sectionColors}</h2>
        <div className="row">
          <label>{t.textColor}</label>
          <input
            type="color"
            value={s.textColor}
            onChange={(e) =>
              onUpdate({ textColor: e.target.value, preset: null })
            }
          />
        </div>
        <div className="row">
          <label>{t.borderColor}</label>
          <input
            type="color"
            value={s.borderColor}
            onChange={(e) =>
              onUpdate({ borderColor: e.target.value, preset: null })
            }
          />
        </div>
        <div className="row">
          <label>{t.textShadow}</label>
          <input
            type="checkbox"
            checked={s.textShadow}
            onChange={(e) =>
              onUpdate({ textShadow: e.target.checked, preset: null })
            }
          />
        </div>
        {s.textShadow && (
          <div className="row">
            <label>{t.shadowColor}</label>
            <input
              type="color"
              value={s.textShadowColor}
              onChange={(e) =>
                onUpdate({ textShadowColor: e.target.value, preset: null })
              }
            />
          </div>
        )}
      </section>

      {/* BORDER & SHAPE */}
      <section className="section">
        <h2 className="section-title">{t.sectionBorderShape}</h2>
        <div className="row">
          <label>{t.borderWidth(s.borderWidth)}</label>
          <input
            type="range"
            min={0}
            max={6}
            value={s.borderWidth}
            onChange={(e) =>
              onUpdate({ borderWidth: Number(e.target.value), preset: null })
            }
          />
        </div>
        <div className="row">
          <label>{t.borderRadius(s.borderRadius)}</label>
          <input
            type="range"
            min={0}
            max={32}
            value={s.borderRadius}
            onChange={(e) =>
              onUpdate({ borderRadius: Number(e.target.value), preset: null })
            }
          />
        </div>
      </section>

      {/* BACKGROUND */}
      <section className="section">
        <h2 className="section-title">{t.sectionBackground}</h2>
        <div className="row">
          <label>{t.blur(s.backgroundBlur)}</label>
          <input
            type="range"
            min={0}
            max={24}
            value={s.backgroundBlur}
            onChange={(e) =>
              onUpdate({ backgroundBlur: Number(e.target.value), preset: null })
            }
          />
        </div>
        <div className="row">
          <label>{t.bgColor}</label>
          <input
            type="color"
            value={
              s.backgroundColor.startsWith("rgba")
                ? rgbaToHex(s.backgroundColor)
                : s.backgroundColor
            }
            onChange={(e) =>
              onUpdate({ backgroundColor: e.target.value, preset: null })
            }
          />
        </div>
      </section>

      {/* BEHAVIOR */}
      <section className="section">
        <h2 className="section-title">{t.sectionBehavior}</h2>
        <div className="row">
          <label>{t.keyDuration(s.keyDisplayDuration)}</label>
          <input
            type="range"
            min={300}
            max={5000}
            step={100}
            value={s.keyDisplayDuration}
            onChange={(e) =>
              onUpdate({
                keyDisplayDuration: Number(e.target.value),
                preset: null,
              })
            }
          />
        </div>
        <div className="row">
          <label>{t.comboMode}</label>
          <input
            type="checkbox"
            checked={s.comboMode}
            onChange={(e) =>
              onUpdate({ comboMode: e.target.checked, preset: null })
            }
          />
        </div>
        <div className="row">
          <label>{t.showModifiersAlone}</label>
          <input
            type="checkbox"
            checked={s.showModifiersAlone}
            onChange={(e) =>
              onUpdate({ showModifiersAlone: e.target.checked, preset: null })
            }
          />
        </div>
        <div className="row">
          <label>{t.showMouseClicks}</label>
          <input
            type="checkbox"
            checked={s.showMouseClicks}
            onChange={(e) =>
              onUpdate({ showMouseClicks: e.target.checked, preset: null })
            }
          />
        </div>
        <div className="row">
          <label>{t.showClickCombos}</label>
          <input
            type="checkbox"
            checked={s.showMouseClickCombos}
            onChange={(e) =>
              onUpdate({
                showMouseClickCombos: e.target.checked,
                preset: null,
              })
            }
          />
        </div>
        <div className="row">
          <label>{t.maxVisibleKeys(s.maxVisibleKeys)}</label>
          <input
            type="range"
            min={1}
            max={10}
            value={s.maxVisibleKeys}
            onChange={(e) =>
              onUpdate({ maxVisibleKeys: Number(e.target.value), preset: null })
            }
          />
        </div>
      </section>

      {/* LAYOUT */}
      <section className="section">
        <h2 className="section-title">{t.sectionLayout}</h2>
        <div className="row">
          <label>{t.layout}</label>
          <select
            value={s.layout}
            onChange={(e) =>
              onUpdate({
                layout: e.target.value as LayoutDirection,
                preset: null,
              })
            }
          >
            <option value="horizontal">{t.layoutHorizontal}</option>
            <option value="vertical">{t.layoutVertical}</option>
          </select>
        </div>
        <div className="row">
          <label>{t.animation}</label>
          <select
            value={s.animationStyle}
            onChange={(e) =>
              onUpdate({
                animationStyle: e.target.value as AnimationStyle,
                preset: null,
              })
            }
          >
            <option value="fade">{t.animFade}</option>
            <option value="slide">{t.animSlide}</option>
            <option value="pop">{t.animPop}</option>
          </select>
        </div>
      </section>
    </div>
  );
}

function rgbaToHex(rgba: string): string {
  const match = rgba.match(/rgba?\((\d+),\s*(\d+),\s*(\d+)/);
  if (!match) return "#000000";
  const r = parseInt(match[1]).toString(16).padStart(2, "0");
  const g = parseInt(match[2]).toString(16).padStart(2, "0");
  const b = parseInt(match[3]).toString(16).padStart(2, "0");
  return `#${r}${g}${b}`;
}
