import { useSettings } from "./hooks/useSettings";
import { SettingsPanel } from "./components/SettingsPanel";
import { StatusBar } from "./components/StatusBar";
import { OverlayUrlCard } from "./components/OverlayUrlCard";
import { LangProvider, useLang } from "./i18n/LangContext";
import "./styles/app.css";

function AppInner() {
  const { settings, updateSettings, loaded } = useSettings();
  const { lang, setLang, t } = useLang();

  if (!loaded) {
    return (
      <div className="loading">
        <span>{t.loading}</span>
      </div>
    );
  }

  return (
    <div className="app">
      <header className="app-header">
        <div className="app-logo">
          <span className="logo-icon">⌨️</span>
          <span className="logo-text">{t.logoText}</span>
        </div>
        <div className="header-right">
          <div className="lang-toggle">
            <button
              className={`lang-btn${lang === "en" ? " active" : ""}`}
              onClick={() => setLang("en")}
            >
              EN
            </button>
            <span className="lang-sep">|</span>
            <button
              className={`lang-btn${lang === "es" ? " active" : ""}`}
              onClick={() => setLang("es")}
            >
              ES
            </button>
          </div>
          <StatusBar />
        </div>
      </header>

      <main className="app-main">
        <OverlayUrlCard settings={settings} onUpdate={updateSettings} />
        <SettingsPanel settings={settings} onUpdate={updateSettings} />
      </main>
    </div>
  );
}

export default function App() {
  return (
    <LangProvider>
      <AppInner />
    </LangProvider>
  );
}
