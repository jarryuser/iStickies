<div align="center">

# iStickies

**Стікери на робочий стіл з vim-душею - окремі прикріплені вікна, Markdown-перегляд і редагування у власному $EDITOR**

[![Tauri](https://img.shields.io/badge/Tauri-2-24C8DB?style=flat-square&logo=tauri&logoColor=white)](https://tauri.app/)
[![Rust](https://img.shields.io/badge/Rust-1.97-CE412B?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![macOS](https://img.shields.io/badge/macOS-supported-000?style=flat-square&logo=apple&logoColor=white)](#)
[![Linux](https://img.shields.io/badge/Linux-supported-FCC624?style=flat-square&logo=linux&logoColor=black)](#)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)](#license)

[English](README.md) · [Slovenčina](README.sk.md) · [Deutsch](README.de.md) · **Українська** · [Русский](README.ru.md)

</div>

---

## Огляд

Stickies - маленький застосунок для стікерів, що живуть просто на шпалерах. Кожна нотатка - окреме безрамкове вікно: тягни куди хочеш, масштабуй за кутик, а вікно лишається під застосунками замість висіти поверх

Редагування відбувається у vim, а не в текстовому полі. Тисни **vim** (або двічі клацни нотатку) і нотатка відкриється у твоєму `$EDITOR` у власному терміналі - Ghostty, iTerm2, Alacritty, kitty, WezTerm або стандартний Terminal. Збережи файл і вміст за секунду синхронізується назад у нотатку, відрендерений як Markdown

---

## Можливості

| | Можливість | Деталі |
|---|---|---|
| 📌 | **Прикріплений вигляд** | Канцелярська кнопка з власним кольором, тінь паперу, кольорові теми нотаток |
| 🖥️ | **Десктопні вікна** | Окремі always-on-bottom вікна, приховані з таскбара і Dock |
| ⌨️ | **Редагування у vim** | `$EDITOR` / `$VISUAL` або ручне налаштування, кожне `:w` синхронізується |
| 🖨️ | **Вибір термінала** | Terminal, iTerm2, Ghostty, Alacritty, kitty, WezTerm, автовизначення |
| 📝 | **Markdown-перегляд** | Заголовки, списки, код, таблиці, `~~закреслений~~`, офлайн-рендеринг |
| 🍔 | **Застосунок меню-бару** | Без іконки в Dock, tray-меню з New / Show all / Hide all / Quit |
| 🖱️ | **Гарячі кути** | Нотатки переживають macOS Show Desktop завдяки Stationary-поведінці вікон |
| 🎨 | **Зовнішній вигляд** | Кольорові зразки нотаток, кнопка вкл/викл, вибір кольору кнопки |

---

## Технології

| Шар | Інструмент | Чому |
|---|---|---|
| Shell | Tauri 2 | Нативні вікна, трей, єдиний 2.5MB бінарник |
| Back end | Rust | Сховище нотаток, vim watcher-потік, запускач терміналів |
| Front end | Vanilla JS + CSS | Без фреймворку, нуль npm-залежностей |
| Markdown | marked 4.3 | Локально в комплекті, працює офлайн |
| Сховище | JSON-файли | `notes.json` + `settings.json` у теці даних застосунку |

---

## Встановлення

Забери `.dmg` (macOS) або `.deb` / `.AppImage` (Linux) з [Releases](https://github.com/jarryuser/iStickies/releases)

macOS-збірки ad-hoc підписані, якщо система каже про пошкодження - виконай `xattr -cr /Applications/iStickies.app`

Якщо macOS вже каже про пошкоджений встановлений застосунок, зніми quarantine-флаг і відкрий знову

```bash
xattr -cr /Applications/iStickies.app
```

---

## Залежності

### macOS

Command Line Tools, Rust і Node

```bash
xcode-select --install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
brew install node
```

### Ubuntu / Debian

Системний webview, компілятор і tray-бібліотеки з apt

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Node 18+ через [nvm](https://github.com/nvm-sh/nvm), fnm або nodejs.org

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

Повний список за дистрибутивом - у [Tauri prerequisites guide](https://tauri.app/start/prerequisites/)

---

## Початок роботи

```bash
npm install
npm run dev      # debug-запуск з hot reload
npm run build    # → iStickies.app + .dmg у src-tauri/target/release/bundle
```

Нотатки й налаштування лежать у `~/Library/Application Support/com.dmytrofiliurskyi.stickies` (на Linux `~/.local/share/stickies`), теку нотаток можна перенести куди завгодно в налаштуваннях

---

## Структура проєкту

```
stickies/
├── src/
│   ├── index.html / main.js   - вікно менеджера: список, кольори, налаштування
│   ├── note.html / note.js    - вікно нотатки: лише показ, drag + resize
│   ├── styles.css             - стилі паперу, кнопки й markdown
│   └── vendor/marked.min.js   - офлайн markdown-рендерер
├── src-tauri/
│   ├── src/lib.rs             - команди: нотатки, bounds, vim, tray, вікна
│   ├── capabilities/          - набори дозволів Tauri
│   ├── tauri.conf.json        - вікна, bundle й метадані застосунку
│   └── icons/                 - іконки застосунку й трея
└── package.json               - tauri / dev / build скрипти
```

---

## Roadmap

### Готово

- [x] **Десктопні вікна** - безрамкові always-on-bottom стікери з drag і resize-кутиком
- [x] **Редагування у vim** - `$EDITOR` у власному терміналі, кожен save синхронізується
- [x] **Вибір термінала** - автовизначений список плюс власна команда
- [x] **Markdown-перегляд** - офлайн-рендеринг із закресленням, таблицями й кодом
- [x] **Вбудований редактор** - inline-правки markdown-джерела через Ctrl+Enter
- [x] **Застосунок меню-бару** - tray-меню, без Dock-іконки, червоний хрестик ховає замість quit
- [x] **Прикріплений вигляд** - кнопка вкл/викл, колір кнопки, кольори нотаток
- [x] **Власна тека** - тека нотаток куди завгодно з міграцією
- [x] **Show Desktop safe** - Stationary-вікна переживають macOS гарячий кут
- [x] **Release CI** - draft-релізи для macOS і Linux за version-тегами

### Ідеї

- [ ] **Windows-підтримка** - vim-флоу і запуск термінала на Windows
- [ ] **Підписані macOS-збірки** - Developer ID і нотарізація, ніякого `xattr`
- [ ] **Глобальний хоткей** - нова нотатка звідки завгодно без менеджера
- [ ] **Автозапуск** - старт при вході з відновленими нотатками
- [ ] **Пошук** - повнотекстовий пошук нотатками з менеджера

---

## Відомі обмеження

- **Wayland** віддає always-on-bottom композитору, нотатки можуть поводитись як звичайні вікна
- **Windows**-підтримка vim-флоу поки заглушка, спочатку macOS і Linux
- **Перший запуск** ad-hoc підписаний, зніми quarantine через `xattr -cr` (див. Встановлення)
- **Markdown-зображення** з мережі не вантажаться, рендерер свідомо офлайн

---

## Ліцензія

MIT © Dmytro Filiurskyi
