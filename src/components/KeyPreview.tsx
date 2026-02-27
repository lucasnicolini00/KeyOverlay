import { AppSettings } from "../types/settings";
import "../styles/key-preview.css";

interface Props {
  settings: AppSettings;
}

const DEMO_KEYS = ["Ctrl", "Shift", "C"];

export function KeyPreview({ settings: s }: Props) {
  const badgeStyle: React.CSSProperties = {
    fontFamily: s.fontFamily,
    fontSize: s.fontSize * 0.6, // scaled down for preview
    color: s.textColor,
    border: `${s.borderWidth}px solid ${s.borderColor}`,
    borderRadius: s.borderRadius,
    background: s.backgroundColor,
    backdropFilter: s.backgroundBlur > 0 ? `blur(${s.backgroundBlur}px)` : undefined,
    textShadow: s.textShadow ? `0 0 8px ${s.textShadowColor}` : undefined,
    padding: "4px 10px",
    display: "inline-flex",
    alignItems: "center",
    whiteSpace: "nowrap" as const,
  };

  const comboLabel = DEMO_KEYS.join("+");

  return (
    <div
      className="key-preview"
      style={{
        flexDirection: s.layout === "horizontal" ? "row" : "column",
      }}
    >
      <div style={{ background: "#111", borderRadius: 8, padding: 12, display: "inline-flex", gap: 8 }}>
        {s.comboMode ? (
          <span style={badgeStyle}>{comboLabel}</span>
        ) : (
          DEMO_KEYS.map((k) => (
            <span key={k} style={badgeStyle}>
              {k}
            </span>
          ))
        )}
      </div>
      <p className="preview-hint">
        Preview at {Math.round(s.fontSize * 0.6)}px (actual overlay uses {s.fontSize}px)
      </p>
    </div>
  );
}
