# KeyOverlay

Overlay de teclas y clics del ratón para streamers y creadores de contenido.  
**Funciona al instante en Windows — sin permisos, sin configuración, solo ejecuta y transmite.**

Diseñado para usarse como **Browser Source en OBS** apuntando a `http://localhost:9002`.

> 🇬🇧 [English version](README.en.md)

![KeyOverlay screenshot](docs/screenshot.png)

---

## Inicio rápido (Windows)

1. **Descarga** el instalador `.exe` desde [Releases](../../releases)
2. Ejecuta el instalador — Windows puede mostrar una advertencia de SmartScreen, haz clic en **"Más información → Ejecutar de todas formas"**
3. Abre **KeyOverlay**
4. Haz clic en **Iniciar captura**
5. En OBS, agrega un **Browser Source** → URL: `http://localhost:9002`
6. ¡Listo! ✅

> Sin derechos de administrador, sin controladores, sin software adicional.

---

## Configuración del Browser Source en OBS

| Ajuste                                 | Valor                   |
| -------------------------------------- | ----------------------- |
| URL                                    | `http://localhost:9002` |
| Ancho                                  | `1920`                  |
| Alto                                   | `120`                   |
| Apagar fuente cuando no sea visible    | **OFF**                 |
| Actualizar navegador al activar escena | **ON**                  |
| CSS personalizado                      | _(dejar vacío)_         |

---

## Características

- **Teclas en tiempo real** — badges animados para cada pulsación
- **Filtro de teclas** — elige exactamente qué teclas quieres mostrar; el resto se ignoran silenciosamente
- **Clics del ratón** — badges LClick / RClick, con combinaciones opcionales (`Ctrl+LClick`)
- **Modo combinación** — muestra los modificadores junto a cada tecla (`Ctrl+Shift+K`)
- **Modificadores solos** — opción para mostrar pulsaciones de solo Ctrl / Alt / Shift / Win
- **Sincronización instantánea** — los cambios de ajustes se reflejan en OBS al momento
- **Varios estilos** — Minimal, Gaming, Retro, Neon
- **Ajustes persistentes** — se recuerdan entre sesiones

---

## Referencia de ajustes

| Ajuste                            | Descripción                                                                               |
| --------------------------------- | ----------------------------------------------------------------------------------------- |
| **Filtro de teclas**              | Lista de teclas permitidas (separadas por comas). Solo esas teclas aparecen en el overlay |
| **Modo combinación**              | Muestra los modificadores junto a cada tecla (`Ctrl+Shift+K`)                             |
| **Mostrar modificadores solos**   | Muestra un badge cuando solo se pulsa Ctrl / Alt / Shift                                  |
| **Mostrar clics del ratón**       | Muestra badges LClick / RClick sin modificadores                                          |
| **Mostrar combinaciones de clic** | Muestra `Ctrl+LClick` cuando se pulsa con un modificador                                  |
| **Distribución**                  | Badges en horizontal o vertical                                                           |
| **Animación**                     | `pop`, `desvanecer` o `deslizar`                                                          |
| **Teclas visibles**               | Cantidad máxima de badges en pantalla                                                     |
| **Duración de tecla**             | Tiempo que se muestra cada badge (ms)                                                     |

---

## Estilos

| Estilo    | Animación  | Combinaciones | Combos de clic |
| --------- | ---------- | ------------- | -------------- |
| ○ Minimal | Pop        | ✅            | ✅             |
| 🎮 Gaming | Pop        | ✅            | ✅             |
| 👾 Retro  | Desvanecer | ✅            | ✅             |
| ✨ Neon   | Pop        | ✅            | ✅             |

---

## Filtro de teclas — muestra solo lo que quieres

KeyOverlay te permite elegir **exactamente qué teclas aparecen en el overlay**. Las que no estén en la lista se ignoran por completo — nunca llegan al stream.

Esto es ideal para:

- 🎮 **Juegos** — mostrar solo las teclas de movimiento y habilidades (`W, A, S, D, Q, E, R, F`)
- 🕹️ **MOBAs / shooters** — filtrar números de habilidades (`1, 2, 3, 4, 5, 6`)
- 📺 **Privacidad** — evitar que contraseñas u otras pulsaciones accidentales aparezcan en pantalla

**Cómo usarlo:**

1. En la sección **OBS Browser Source** de la app, activa **Filtro de teclas**
2. Escribe las teclas que quieres mostrar, separadas por comas
3. Los cambios se aplican al instante — sin reiniciar OBS

**Ejemplos:**

| Uso                          | Filtro                  |
| ---------------------------- | ----------------------- |
| WASD + habilidades           | `W,A,S,D,Q,E,R,F`       |
| Números de habilidades       | `1,2,3,4,5,6`           |
| Teclas de movimiento clásico | `W,A,S,D,Space`         |
| Todo (sin filtro)            | _(desactiva el filtro)_ |

> El filtro no distingue entre mayúsculas y minúsculas. `q` y `Q` son equivalentes.

---

## Advertencia de SmartScreen en Windows

Al descargar KeyOverlay, Windows puede mostrar este mensaje:

> _"Windows protegió su PC — Microsoft Defender SmartScreen impidió el inicio de una aplicación no reconocida."_

Esto es **normal y esperado**. No significa que la app sea maliciosa.

**¿Por qué ocurre?**  
Windows exige un **certificado de firma de código** para confiar automáticamente en un ejecutable. Obtener uno cuesta entre $200 y $500 USD al año — un gasto inviable para un proyecto personal gratuito. Por eso KeyOverlay se distribuye sin firmar, como la mayoría de las herramientas indie y de código abierto.

**¿Qué hacer?**  
Haz clic en **"Más información" → "Ejecutar de todas formas"** y la app abrirá normalmente. Solo necesitas hacerlo una vez.

Si prefieres verificarlo tú mismo, el código fuente completo está disponible en este repositorio.

---

## Compilar desde el código fuente

Requisitos: [Node.js](https://nodejs.org/) 18+, [Rust](https://rustup.rs/) 1.77+, [Tauri CLI](https://tauri.app/start/prerequisites/)

```bash
npm install
npm run tauri dev     # desarrollo (hot-reload)
npm run tauri build   # instalador → src-tauri/target/release/bundle/
```

---

## Plataformas

| Plataforma        | Estado              | Notas                                    |
| ----------------- | ------------------- | ---------------------------------------- |
| **Windows 10/11** | ✅ Soporte completo | Sin permisos necesarios                  |
| macOS 12+         | ✅ Compatible       | Requiere permiso de Monitoreo de Entrada |

---

## Licencia

MIT
