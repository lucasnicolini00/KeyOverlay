export type Lang = "en" | "es";

export const translations = {
  en: {
    // App shell
    loading: "Loading KeyOverlay…",
    logoText: "KeyOverlay",

    // OBS card
    obsBrowserSource: "OBS Browser Source",
    requestInputMonitoring: "Request Input Monitoring",
    inputMonitoringGranted: "Input Monitoring granted",
    inputMonitoringHint:
      "Open System Settings → Privacy & Security → Input Monitoring, add KeyOverlay, then click again.",
    startCapture: "Start Key Capture",
    starting: "Starting…",
    stopCapture: "Stop Capturing",
    stopping: "Stopping…",
    deniedHint: "Grant Input Monitoring (step 1) first, then try again.",
    copyUrl: "Copy URL",
    copied: "✓ Copied!",
    open: "Open",
    obsHint:
      "In OBS: Add Source → Browser → paste URL above. Recommended size: 800×160",

    // Settings sections
    sectionPresets: "Presets",
    sectionPreview: "Preview",
    sectionTypography: "Typography",
    sectionColors: "Colors",
    sectionBorderShape: "Border & Shape",
    sectionBackground: "Background",
    sectionBehavior: "Behavior",
    sectionLayout: "Layout & Animation",

    // Reset
    resetBtn: "↺ Reset",
    resetTitle: "Reset all settings to default",

    // Typography
    font: "Font",
    fontSize: (n: number) => `Size — ${n}px`,

    // Colors
    textColor: "Text color",
    borderColor: "Border color",
    textShadow: "Text shadow",
    shadowColor: "Shadow color",

    // Border & Shape
    borderWidth: (n: number) => `Border width — ${n}px`,
    borderRadius: (n: number) => `Border radius — ${n}px`,

    // Background
    blur: (n: number) => `Blur — ${n}px`,
    bgColor: "Background color/alpha",

    // Behavior
    keyDuration: (n: number) => `Key display duration — ${n}ms`,
    comboMode: "Combo mode",
    showModifiersAlone: "Show modifiers alone",
    showMouseClicks: "Show mouse clicks",
    showClickCombos: "Show click combos (e.g. Ctrl+LClick)",
    keyFilterLabel: "Key filter",
    keyFilterPlaceholder:
      "Add or remove the keys you want to show in the overlay",
    maxVisibleKeys: (n: number) => `Max visible keys — ${n}`,

    // Layout & Animation
    layout: "Layout",
    layoutHorizontal: "Horizontal",
    layoutVertical: "Vertical",
    animation: "Animation",
    animFade: "Fade",
    animSlide: "Slide",
    animPop: "Pop",

    // Language toggle
    language: "Language",
  },

  es: {
    // App shell
    loading: "Cargando KeyOverlay…",
    logoText: "KeyOverlay",

    // OBS card
    obsBrowserSource: "Fuente de navegador OBS",
    requestInputMonitoring: "Solicitar permiso de teclado",
    inputMonitoringGranted: "Permiso de teclado concedido",
    inputMonitoringHint:
      "Abre Configuración del Sistema → Privacidad y seguridad → Monitoreo de entrada, agrega KeyOverlay y vuelve a hacer clic.",
    startCapture: "Iniciar captura",
    starting: "Iniciando…",
    stopCapture: "Detener captura",
    stopping: "Deteniendo…",
    deniedHint:
      "Concede el permiso de teclado (paso 1) primero y vuelve a intentarlo.",
    copyUrl: "Copiar URL",
    copied: "✓ ¡Copiado!",
    open: "Abrir",
    obsHint:
      "En OBS: Agrega fuente → Navegador → pega la URL de arriba. Tamaño recomendado: 800×160",

    // Settings sections
    sectionPresets: "Estilos",
    sectionPreview: "Vista previa",
    sectionTypography: "Tipografía",
    sectionColors: "Colores",
    sectionBorderShape: "Borde y forma",
    sectionBackground: "Fondo",
    sectionBehavior: "Comportamiento",
    sectionLayout: "Distribución y animación",

    // Reset
    resetBtn: "↺ Restablecer",
    resetTitle: "Restablecer todos los ajustes",

    // Typography
    font: "Fuente",
    fontSize: (n: number) => `Tamaño — ${n}px`,

    // Colors
    textColor: "Color del texto",
    borderColor: "Color del borde",
    textShadow: "Sombra de texto",
    shadowColor: "Color de sombra",

    // Border & Shape
    borderWidth: (n: number) => `Ancho de borde — ${n}px`,
    borderRadius: (n: number) => `Radio de borde — ${n}px`,

    // Background
    blur: (n: number) => `Desenfoque — ${n}px`,
    bgColor: "Color de fondo / transparencia",

    // Behavior
    keyDuration: (n: number) => `Duración de tecla — ${n}ms`,
    comboMode: "Modo combinación",
    showModifiersAlone: "Mostrar modificadores solos",
    showMouseClicks: "Mostrar clics del ratón",
    showClickCombos: "Mostrar combinaciones de clic (ej. Ctrl+Click)",
    keyFilterLabel: "Filtro de teclas",
    keyFilterPlaceholder:
      "Agrega o elimina las teclas que quieres mostrar en el overlay",
    maxVisibleKeys: (n: number) => `Teclas visibles — ${n}`,

    // Layout & Animation
    layout: "Distribución",
    layoutHorizontal: "Horizontal",
    layoutVertical: "Vertical",
    animation: "Animación",
    animFade: "Desvanecer",
    animSlide: "Deslizar",
    animPop: "Pop",

    // Language toggle
    language: "Idioma",
  },
} as const;

export type Translations = (typeof translations)["en"];
