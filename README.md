<div align="center">

# iStickies

**Desktop sticky notes with vim soul - separate pinned windows, Markdown preview, and editing in your own $EDITOR**

[![Tauri](https://img.shields.io/badge/Tauri-2-24C8DB?style=flat-square&logo=tauri&logoColor=white)](https://tauri.app/)
[![Rust](https://img.shields.io/badge/Rust-1.97-CE412B?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![macOS](https://img.shields.io/badge/macOS-supported-000?style=flat-square&logo=apple&logoColor=white)](#)
[![Linux](https://img.shields.io/badge/Linux-supported-FCC624?style=flat-square&logo=linux&logoColor=black)](#)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)](#license)

**English** · [Slovenčina](README.sk.md) · [Deutsch](README.de.md) · [Українська](README.uk.md) · [Русский](README.ru.md)

</div>

---

## Overview

Stickies is a small desktop app for sticky notes that live directly on your wallpaper. Each note is a separate frameless window: drag it anywhere, resize it by the corner grip, and it stays below your apps instead of floating on top

Editing happens in vim, not in a text field. Press **vim** (or double-click a note) and the note opens in your `$EDITOR` inside your own terminal - Ghostty, iTerm2, Alacritty, kitty, WezTerm or the default Terminal. Save the file and the content syncs back into the note within a second, rendered as Markdown

---

## Features

| | Feature | Details |
|---|---|---|
| 📌 | **Pinned look** | Pushpin with custom color, paper shadow, per-note color themes |
| 🖥️ | **Desktop windows** | Separate always-on-bottom windows, hidden from taskbar and Dock |
| ⌨️ | **Vim editing** | `$EDITOR` / `$VISUAL` or a manual override, every `:w` syncs back |
| 🖨️ | **Terminal choice** | Terminal, iTerm2, Ghostty, Alacritty, kitty, WezTerm, auto-detected |
| 📝 | **Markdown preview** | Headings, lists, code, tables, `~~strikethrough~~`, offline rendering |
| 🍔 | **Menu bar app** | No Dock icon, tray menu with New / Show all / Hide all / Quit |
| 🖱️ | **Hot corner safe** | Notes survive macOS Show Desktop via Stationary window behavior |
| 🎨 | **Appearance** | Note color swatches, pin on/off, pin color picker |

---

## Tech stack

| Layer | Tool | Why |
|---|---|---|
| Shell | Tauri 2 | Native windows, tray, single 2.5MB binary |
| Back end | Rust | Notes store, vim watcher thread, terminal launcher |
| Front end | Vanilla JS + CSS | No framework, zero npm dependencies |
| Markdown | marked 4.3 | Vendored locally, works offline |
| Storage | JSON files | `notes.json` + `settings.json` in the app data dir |

---

## Install

Grab the `.dmg` (macOS) or `.deb` / `.AppImage` (Linux) from [Releases](https://github.com/jarryuser/iStickies/releases)

macOS builds are ad-hoc signed, if the system reports damage run `xattr -cr /Applications/iStickies.app`

If macOS already reports the installed app as damaged, clear the quarantine flag and open it again

```bash
xattr -cr /Applications/iStickies.app
```

---

## Prerequisites

### macOS

Command Line Tools, Rust and Node

```bash
xcode-select --install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
brew install node
```

### Ubuntu / Debian

System webview, compiler toolchain and tray libraries come from apt

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Node 18+ via [nvm](https://github.com/nvm-sh/nvm), fnm or nodejs.org

### Fedora

```bash
sudo dnf install webkit2gtk4.1-devel openssl-devel curl wget file \
  libxdo-devel libayatana-appindicator-gtk3-devel librsvg2-devel
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Arch

```bash
sudo pacman -S webkit2gtk-4.1 base-devel curl wget file openssl \
  libappindicator-gtk3 librsvg
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Full per-distro list lives in the [Tauri prerequisites guide](https://tauri.app/start/prerequisites/)

---

## Getting started

```bash
npm install
npm run dev      # debug run with hot reload
npm run build    # → iStickies.app + .dmg under src-tauri/target/release/bundle
```

Notes and settings live in `~/Library/Application Support/com.dmytrofiliurskyi.stickies` (`~/.local/share/stickies` on Linux)

---

## Project structure

```
stickies/
├── src/
│   ├── index.html / main.js   - manager window: list, colors, settings
│   ├── note.html / note.js    - sticky window: display only, drag + resize
│   ├── styles.css             - paper, pushpin and markdown styles
│   └── vendor/marked.min.js   - offline markdown renderer
├── src-tauri/
│   ├── src/lib.rs             - commands: notes, bounds, vim, tray, windows
│   ├── capabilities/          - Tauri permission sets
│   ├── tauri.conf.json        - windows, bundle and app metadata
│   └── icons/                 - app and tray icons
└── package.json               - tauri / dev / build scripts
```

---

## Known limitations

- **Wayland** leaves always-on-bottom to the compositor, notes may behave like normal windows
- **Windows** support for the vim flow is stubbed, macOS and Linux come first
- **First launch** is ad-hoc signed, clear quarantine with `xattr -cr` (see Install)
- **Markdown images** from the network do not load, the renderer is offline by design

---

## License

MIT © Dmytro Filiurskyi
