<div align="center">

# iStickies

**Lepiace poznámky s vim dušou - samostatné pripnuté okná, Markdown náhľad a úpravy vo vlastnom $EDITOR**

[![Tauri](https://img.shields.io/badge/Tauri-2-24C8DB?style=flat-square&logo=tauri&logoColor=white)](https://tauri.app/)
[![Rust](https://img.shields.io/badge/Rust-1.97-CE412B?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![macOS](https://img.shields.io/badge/macOS-supported-000?style=flat-square&logo=apple&logoColor=white)](#)
[![Linux](https://img.shields.io/badge/Linux-supported-FCC624?style=flat-square&logo=linux&logoColor=black)](#)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)](#license)

[English](README.md) · **Slovenčina** · [Deutsch](README.de.md) · [Українська](README.uk.md) · [Русский](README.ru.md)

</div>

---

## Prehľad

Stickies je malá desktopová aplikácia na lepiace poznámky, ktoré žijú priamo na tapete. Každá poznámka je samostatné bezrámové okno: presuň ho kamkoľvek, zmeň veľkosť za roh a okno zostane pod aplikáciami namiesto plávania navrchu

Úpravy prebiehajú vo vime, nie v textovom poli. Stlač **vim** (alebo dvakrát klikni na poznámku) a poznámka sa otvorí vo tvojom `$EDITOR` vo vlastnom termináli - Ghostty, iTerm2, Alacritty, kitty, WezTerm alebo predvolený Terminal. Ulož súbor a obsah sa do sekundy synchronizuje späť do poznámky, vyrenderovaný ako Markdown

---

## Funkcie

| | Funkcia | Detaily |
|---|---|---|
| 📌 | **Pripnutý vzhľad** | Špendlík s vlastnou farbou, papierový tieň, farebné témy poznámok |
| 🖥️ | **Desktopové okná** | Samostatné okná vždy naspodku, skryté z taskbaru aj Docku |
| ⌨️ | **Úpravy vo vime** | `$EDITOR` / `$VISUAL` alebo ručné nastavenie, každé `:w` sa synchronizuje |
| 🖨️ | **Voľba terminálu** | Terminal, iTerm2, Ghostty, Alacritty, kitty, WezTerm, autodetekcia |
| 📝 | **Markdown náhľad** | Nadpisy, zoznamy, kód, tabuľky, `~~preškrtnuté~~`, offline renderovanie |
| 🍔 | **Menu bar aplikácia** | Bez ikony v Docku, tray menu s New / Show all / Hide all / Quit |
| 🖱️ | **Horúce rohy** | Poznámky prežijú macOS Show Desktop vďaka Stationary správaniu okien |
| 🎨 | **Vzhľad** | Farebné vzorky poznámok, špendlík zap/vyp, výber farby špendlíka |

---

## Technológie

| Vrstva | Nástroj | Prečo |
|---|---|---|
| Shell | Tauri 2 | Natívne okná, tray, jediná 2.5MB binárka |
| Back end | Rust | Úložisko poznámok, vim watcher vlákno, spúšťač terminálov |
| Front end | Vanilla JS + CSS | Bez frameworku, nula npm závislostí |
| Markdown | marked 4.3 | Lokálne vendored, funguje offline |
| Úložisko | JSON súbory | `notes.json` + `settings.json` v app data priečinku |

---

## Inštalácia

Stiahni `.dmg` (macOS) alebo `.deb` / `.AppImage` (Linux) z [Releases](https://github.com/jarryuser/iStickies/releases)

macOS buildy sú ad-hoc podpísané, ak systém hlási poškodenie spusti `xattr -cr /Applications/iStickies.app`

Ak macOS už nainštalovanú aplikáciu hlási ako poškodenú, vyčisti quarantine flag a otvor ju znova

```bash
xattr -cr /Applications/iStickies.app
```

---

## Predpoklady

### macOS

Command Line Tools, Rust a Node

```bash
xcode-select --install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
brew install node
```

### Ubuntu / Debian

Systémový webview, toolchain prekladača a tray knižnice z apt

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Node 18+ cez [nvm](https://github.com/nvm-sh/nvm), fnm alebo nodejs.org

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

Úplný zoznam podľa distribúcie je v [Tauri prerequisites guide](https://tauri.app/start/prerequisites/)

---

## Začíname

```bash
npm install
npm run dev      # debug beh s hot reload
npm run build    # → iStickies.app + .dmg v src-tauri/target/release/bundle
```

Poznámky a nastavenia ležia v `~/Library/Application Support/com.dmytrofiliurskyi.stickies` (na Linuxe `~/.local/share/stickies`), priečinok poznámok presunieš kdekoľvek v nastaveniach

---

## Štruktúra projektu

```
stickies/
├── src/
│   ├── index.html / main.js   - okno manažéra: zoznam, farby, nastavenia
│   ├── note.html / note.js    - okno poznámky: len zobrazenie, drag + resize
│   ├── styles.css             - papier, špendlík a markdown štýly
│   └── vendor/marked.min.js   - offline markdown renderer
├── src-tauri/
│   ├── src/lib.rs             - príkazy: poznámky, bounds, vim, tray, okná
│   ├── capabilities/          - Tauri permission sety
│   ├── tauri.conf.json        - okná, bundle a metadata aplikácie
│   └── icons/                 - ikony aplikácie a traye
└── package.json               - tauri / dev / build skripty
```

---

## Roadmap

### Hotovo

- [x] **Desktopové okná** - bezrámové always-on-bottom poznámky s dragom a resize rohom
- [x] **Úpravy vo vime** - `$EDITOR` vo vlastnom termináli, každý save sa synchronizuje
- [x] **Voľba terminálu** - autodetekovaný zoznam plus vlastný príkaz
- [x] **Markdown náhľad** - offline renderovanie s preškrtnutím, tabuľkami a kódom
- [x] **Zabudovaný editor** - inline úpravy markdown zdroja cez Ctrl+Enter
- [x] **Menu bar aplikácia** - tray menu, bez Dock ikony, červený krížik skryje namiesto quit
- [x] **Pripnutý vzhľad** - špendlík zap/vyp, farba špendlíka, farby poznámok
- [x] **Vlastné úložisko** - priečinok poznámok kamkoľvek s migráciou
- [x] **Show Desktop safe** - Stationary okná prežijú macOS horúci roh
- [x] **Release CI** - draft releasy pre macOS a Linux na version tagy

### Nápady

- [ ] **Windows podpora** - vim flow a štart terminálu na Windows
- [ ] **Podpísané macOS buildy** - Developer ID a notarizácia, žiadne `xattr`
- [ ] **Globálna hotkey** - nová poznámka odkiaľkoľvek bez otvárania manažéra
- [ ] **Autostart** - štart pri prihlásení s obnovenými poznámkami
- [ ] **Vyhľadávanie** - fulltext cez poznámky z manažéra

---

## Známe obmedzenia

- **Wayland** prenecháva always-on-bottom kompozitoru, poznámky sa môžu správať ako bežné okná
- **Windows** podpora vim flow je zatiaľ stub, najprv macOS a Linux
- **Prvé spustenie** je ad-hoc podpísané, vyčisti quarantine cez `xattr -cr` (viď Inštalácia)
- **Markdown obrázky** zo siete sa nenačítavajú, renderer je zámerne offline

---

## Licencia

MIT © Dmytro Filiurskyi
