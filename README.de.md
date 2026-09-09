<div align="center">

# iStickies

**Desktop-Notizzettel mit Vim-Seele - einzelne angeheftete Fenster, Markdown-Vorschau und Bearbeiten im eigenen $EDITOR**

[![Tauri](https://img.shields.io/badge/Tauri-2-24C8DB?style=flat-square&logo=tauri&logoColor=white)](https://tauri.app/)
[![Rust](https://img.shields.io/badge/Rust-1.97-CE412B?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![macOS](https://img.shields.io/badge/macOS-supported-000?style=flat-square&logo=apple&logoColor=white)](#)
[![Linux](https://img.shields.io/badge/Linux-supported-FCC624?style=flat-square&logo=linux&logoColor=black)](#)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)](#license)

[English](README.md) · [Slovenčina](README.sk.md) · **Deutsch** · [Українська](README.uk.md) · [Русский](README.ru.md)

</div>

---

## Überblick

Stickies ist eine kleine Desktop-App für Notizzettel, die direkt auf dem Hintergrundbild leben. Jede Notiz ist ein eigenes rahmenloses Fenster: beliebig positionieren, per Eckgriff skalieren, und es bleibt unter den Apps statt darüber zu schweben

Bearbeitet wird in Vim, nicht in einem Textfeld. Drücke **vim** (oder doppelklicke eine Notiz) und die Notiz öffnet sich in deinem `$EDITOR` im eigenen Terminal - Ghostty, iTerm2, Alacritty, kitty, WezTerm oder das Standard-Terminal. Datei speichern und der Inhalt synct sich binnen einer Sekunde zurück in die Notiz, gerendert als Markdown

---

## Funktionen

| | Funktion | Details |
|---|---|---|
| 📌 | **Pinnwand-Look** | Reißzwecke mit eigener Farbe, Papierschatten, Farbthemes pro Notiz |
| 🖥️ | **Desktop-Fenster** | Eigene Always-on-bottom-Fenster, versteckt aus Taskleiste und Dock |
| ⌨️ | **Vim-Bearbeitung** | `$EDITOR` / `$VISUAL` oder manuell gesetzt, jedes `:w` synct zurück |
| 🖨️ | **Terminalwahl** | Terminal, iTerm2, Ghostty, Alacritty, kitty, WezTerm, Auto-Erkennung |
| 📝 | **Markdown-Vorschau** | Headings, Listen, Code, Tabellen, `~~durchgestrichen~~`, offline |
| 🍔 | **Menüleisten-App** | Kein Dock-Icon, Tray-Menü mit New / Show all / Hide all / Quit |
| 🖱️ | **Heiße Ecken** | Notizen überleben macOS Schreibtisch zeigen via Stationary-Verhalten |
| 🎨 | **Aussehen** | Farbfelder pro Notiz, Pin an/aus, Pin-Farbwähler |

---

## Tech-Stack

| Schicht | Tool | Warum |
|---|---|---|
| Shell | Tauri 2 | Native Fenster, Tray, ein einziges 2.5MB-Binary |
| Back end | Rust | Notizspeicher, Vim-Watcher-Thread, Terminal-Starter |
| Front end | Vanilla JS + CSS | Kein Framework, null npm-Abhängigkeiten |
| Markdown | marked 4.3 | Lokal mitgeliefert, läuft offline |
| Ablage | JSON-Dateien | `notes.json` + `settings.json` im App-Datenordner |

---

## Einstieg

```bash
npm install
npm run dev      # Debug-Lauf mit Hot Reload
npm run build    # → stickies.app + .dmg unter src-tauri/target/release/bundle
```

Braucht Rust 1.97+, Node 18+ und Xcode Command Line Tools unter macOS. Notizen und Einstellungen liegen in `~/Library/Application Support/com.dmytrofiliurskyi.stickies` (unter Linux `~/.local/share/stickies`).

---

## Projektstruktur

```
stickies/
├── src/
│   ├── index.html / main.js   - Manager-Fenster: Liste, Farben, Einstellungen
│   ├── note.html / note.js    - Notizfenster: nur Anzeige, Drag + Resize
│   ├── styles.css             - Papier-, Pin- und Markdown-Stile
│   └── vendor/marked.min.js   - Offline-Markdown-Renderer
├── src-tauri/
│   ├── src/lib.rs             - Kommandos: Notizen, Bounds, Vim, Tray, Fenster
│   ├── capabilities/          - Tauri-Berechtigungen
│   ├── tauri.conf.json        - Fenster, Bundle- und App-Metadaten
│   └── icons/                 - App- und Tray-Icons
└── package.json               - tauri / dev / build Skripte
```

---

## Bekannte Grenzen

- **Wayland** überlässt Always-on-bottom dem Compositor, Notizen wirken evtl. wie normale Fenster
- **Windows**-Unterstützung für den Vim-Flow ist vorerst ein Stub, macOS und Linux zuerst
- **Erststart** ist unsigniert, Rechtsklick → Öffnen oder Quarantine-Flag entfernen
- **Markdown-Bilder** aus dem Netz laden nicht, der Renderer ist bewusst offline

---

## Lizenz

MIT © Dmytro Filiurskyi
