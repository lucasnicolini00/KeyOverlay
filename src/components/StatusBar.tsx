import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import "../styles/status-bar.css";

export function StatusBar() {
  const [connected, setConnected] = useState(0);
  const [lastKey, setLastKey] = useState<string>("");

  useEffect(() => {
    const unlistenConnected = listen<number>("overlay-clients-changed", (e) => {
      setConnected(e.payload);
    });
    const unlistenKey = listen<string>("key-pressed", (e) => {
      setLastKey(e.payload);
    });
    return () => {
      unlistenConnected.then((f) => f());
      unlistenKey.then((f) => f());
    };
  }, []);

  return (
    <div className="status-bar">
      <span className={`ws-dot ${connected > 0 ? "connected" : ""}`} />
      <span className="ws-label">
        {connected > 0
          ? `${connected} overlay${connected > 1 ? "s" : ""} connected`
          : "No overlay connected"}
      </span>
      {lastKey && (
        <span className="last-key">
          Last: <kbd>{lastKey}</kbd>
        </span>
      )}
    </div>
  );
}
