import { createContext, useContext, useState, ReactNode } from "react";
import { Lang, translations, Translations } from "./translations";

interface LangContextValue {
  lang: Lang;
  setLang: (l: Lang) => void;
  t: Translations;
}

function detectLang(): Lang {
  // If the user previously chose a language, honour it
  const stored = localStorage.getItem("lang");
  if (stored === "en" || stored === "es") return stored;

  // Otherwise, detect from the OS/browser locale
  const locale = navigator.language || "en";
  return locale.toLowerCase().startsWith("es") ? "es" : "en";
}

const LangContext = createContext<LangContextValue>({
  lang: "en",
  setLang: () => {},
  t: translations.en,
});

export function LangProvider({ children }: { children: ReactNode }) {
  const [lang, setLangState] = useState<Lang>(detectLang);

  const setLang = (l: Lang) => {
    localStorage.setItem("lang", l);
    setLangState(l);
  };

  return (
    <LangContext.Provider
      value={{ lang, setLang, t: translations[lang] as Translations }}
    >
      {children}
    </LangContext.Provider>
  );
}

export function useLang() {
  return useContext(LangContext);
}
