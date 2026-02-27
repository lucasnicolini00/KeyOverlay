import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { openUrl } from "@tauri-apps/plugin-opener";
import { useLang } from "../i18n/LangContext";
import { AppSettings } from "../types/settings";
import "../styles/overlay-card.css";

const OVERLAY_URL = "http://localhost:9002";

type CaptureStatus = "idle" | "starting" | "stopping" | "running" | "denied";

interface Props {
  settings: AppSettings;
  onUpdate: (patch: Partial<AppSettings>) => void;
}

export function OverlayUrlCard({ settings, onUpdate }: Props) {
  const { t } = useLang();
  const [copied, setCopied] = useState(false);
  const [accessOk, setAccessOk] = useState<boolean | null>(null);
  const [captureStatus, setCaptureStatus] = useState<CaptureStatus>("idle");
  const [isMac, setIsMac] = useState(true); // assume mac until detected

  useEffect(() => {
    setIsMac(navigator.platform.toLowerCase().includes("mac"));
  }, []);

  // Receive the initial accessibility status emitted by Rust on startup
  useEffect(() => {
    let unlisten: (() => void) | undefined;
    listen<boolean>("accessibility-status", (e) => {
      setAccessOk(e.payload);
    }).then((fn) => {
      unlisten = fn;
    });
    return () => unlisten?.();
  }, []);

  const copyUrl = () => {
    navigator.clipboard.writeText(OVERLAY_URL);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const checkAccess = async () => {
    try {
      const ok = await invoke<boolean>("check_accessibility");
      setAccessOk(ok);
    } catch {
      setAccessOk(false);
    }
  };

  const toggleCapture = async () => {
    if (captureStatus === "running") {
      // Stop
      setCaptureStatus("stopping");
      try {
        await invoke("stop_key_capture");
      } catch (err) {
        console.error("[KeyOverlay] stop_key_capture error:", err);
      }
      setCaptureStatus("idle");
      return;
    }

    // Start
    setCaptureStatus("starting");
    try {
      const result = await invoke<string>("start_key_capture");
      if (result === "started" || result === "already_running") {
        setCaptureStatus("running");
      }
    } catch (err) {
      const msg = typeof err === "string" ? err : String(err);
      if (msg.includes("accessibility_denied")) {
        setCaptureStatus("denied");
      } else {
        setCaptureStatus("idle");
        console.error("[KeyOverlay] start_key_capture error:", err);
      }
    }
  };

  return (
    <div className="overlay-card">
      <div className="overlay-card-title">
        <span className="obs-icon">🎬</span>
        {t.obsBrowserSource}
      </div>

      {/* ── Setup steps ──────────────────────────────────────────────── */}
      <div className={`setup-steps${isMac ? "" : " setup-steps--single"}`}>
        {/* Step 1 — macOS only: Input Monitoring permission */}
        {isMac && (
          <div className="setup-step">
            <button
              className={`step-btn${accessOk === true ? " step-btn--done" : ""}`}
              onClick={checkAccess}
            >
              <span className="step-num">1</span>
              <span className="step-label">
                {accessOk === true
                  ? t.inputMonitoringGranted
                  : t.requestInputMonitoring}
              </span>
              {accessOk === true && <span className="step-check">✓</span>}
            </button>
            {accessOk === false && (
              <p className="step-hint step-hint--err">
                {t.inputMonitoringHint}
              </p>
            )}
          </div>
        )}

        {/* Step 2 on macOS / Step 1 on Windows */}
        <div className="setup-step">
          <button
            className={`step-btn${
              captureStatus === "running"
                ? " step-btn--done"
                : captureStatus === "denied"
                  ? " step-btn--err"
                  : ""
            }`}
            onClick={toggleCapture}
            disabled={
              captureStatus === "starting" || captureStatus === "stopping"
            }
          >
            <span className="step-num">{isMac ? "2" : "1"}</span>
            <span className="step-label">
              {captureStatus === "idle" && t.startCapture}
              {captureStatus === "starting" && t.starting}
              {captureStatus === "running" && t.stopCapture}
              {captureStatus === "stopping" && t.stopping}
              {captureStatus === "denied" && t.startCapture}
            </span>
            {captureStatus === "running" && (
              <span className="step-check step-check--stop">■</span>
            )}
          </button>
          {captureStatus === "denied" && (
            <p className="step-hint step-hint--err">{t.deniedHint}</p>
          )}
        </div>
      </div>

      {/* ── OBS URL ──────────────────────────────────────────────────── */}
      <div className="overlay-url-row">
        <span className="overlay-url">{OVERLAY_URL}</span>
        <button className="copy-btn" onClick={copyUrl}>
          {copied ? t.copied : t.copyUrl}
        </button>
        <button className="copy-btn" onClick={() => openUrl(OVERLAY_URL)}>
          {t.open}
        </button>
      </div>
      <div className="overlay-hint">{t.obsHint}</div>

      {/* ── Key filter ───────────────────────────────────────────────── */}
      <div className="key-filter-row">
        <div className="key-filter-label">
          <input
            type="checkbox"
            id="kf-toggle"
            checked={settings.keyFilterEnabled}
            onChange={(e) =>
              onUpdate({ keyFilterEnabled: e.target.checked, preset: null })
            }
          />
          <label htmlFor="kf-toggle">{t.keyFilterLabel}</label>
          <span className="key-filter-hint">{t.keyFilterPlaceholder}</span>
        </div>
        {settings.keyFilterEnabled && (
          <input
            type="text"
            className="filter-input"
            placeholder="Q,W,E,R,1,2,3,4,5,6"
            value={settings.keyFilter}
            onChange={(e) =>
              onUpdate({ keyFilter: e.target.value, preset: null })
            }
          />
        )}
      </div>
    </div>
  );
}
