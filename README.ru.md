<div align="center">

# iStickies

**Стикеры на рабочий стол с vim-душой - отдельные прикрепленные окна, Markdown-превью и редактура в собственном $EDITOR**

[![Tauri](https://img.shields.io/badge/Tauri-2-24C8DB?style=flat-square&logo=tauri&logoColor=white)](https://tauri.app/)
[![Rust](https://img.shields.io/badge/Rust-1.97-CE412B?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![macOS](https://img.shields.io/badge/macOS-supported-000?style=flat-square&logo=apple&logoColor=white)](#)
[![Linux](https://img.shields.io/badge/Linux-supported-FCC624?style=flat-square&logo=linux&logoColor=black)](#)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)](#license)

[English](README.md) · [Slovenčina](README.sk.md) · [Deutsch](README.de.md) · [Українська](README.uk.md) · **Русский**

</div>

---

## Обзор

Stickies - маленькое приложение для стикеров, живущих прямо на обоях. Каждая заметка - отдельное безрамочное окно: тащи куда хочешь, масштабируй за уголок, а окно остается под приложениями вместо висения поверх

Редактура происходит в vim, а не в текстовом поле. Жми **vim** (или дважды кликни заметку) и заметка откроется в твоем `$EDITOR` в собственном терминале - Ghostty, iTerm2, Alacritty, kitty, WezTerm или стандартный Terminal. Сохрани файл и содержимое за секунду синхронизируется обратно в заметку, отрендеренное как Markdown

---

## Возможности

| | Возможность | Детали |
|---|---|---|
| 📌 | **Прикрепленный вид** | Канцелярская кнопка со своим цветом, тень бумаги, цветовые темы заметок |
| 🖥️ | **Десктопные окна** | Отдельные always-on-bottom окна, скрыты из таскбара и Dock |
| ⌨️ | **Редактура в vim** | `$EDITOR` / `$VISUAL` или ручная настройка, каждое `:w` синхронизируется |
| 🖨️ | **Выбор терминала** | Terminal, iTerm2, Ghostty, Alacritty, kitty, WezTerm, автоопределение |
| 📝 | **Markdown-превью** | Заголовки, списки, код, таблицы, `~~зачеркнутый~~`, офлайн-рендеринг |
| 🍔 | **Приложение меню-бара** | Без иконки в Dock, tray-меню с New / Show all / Hide all / Quit |
| 🖱️ | **Горячие углы** | Заметки переживают macOS Show Desktop благодаря Stationary-поведению окон |
| 🎨 | **Внешний вид** | Цветовые образцы заметок, кнопка вкл/выкл, выбор цвета кнопки |

---

## Технологии

| Слой | Инструмент | Почему |
|---|---|---|
| Shell | Tauri 2 | Нативные окна, трей, единый 2.5MB бинарник |
| Back end | Rust | Хранилище заметок, vim watcher-поток, запускатель терминалов |
| Front end | Vanilla JS + CSS | Без фреймворка, ноль npm-зависимостей |
| Markdown | marked 4.3 | Локально в комплекте, работает офлайн |
| Хранилище | JSON-файлы | `notes.json` + `settings.json` в папке данных приложения |

---

## Установка

Забери `.dmg` (macOS) или `.deb` / `.AppImage` (Linux) из [Releases](https://github.com/jarryuser/iStickies/releases)

macOS-сборки ad-hoc подписаны, если система ругается на повреждение - выполни `xattr -cr /Applications/iStickies.app`

---

## Зависимости

### macOS

Command Line Tools, Rust и Node

```bash
xcode-select --install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
brew install node
```

### Ubuntu / Debian

Системный webview, компилятор и tray-библиотеки из apt

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Node 18+ через [nvm](https://github.com/nvm-sh/nvm), fnm или nodejs.org

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

Полный список по дистрибутивам - в [Tauri prerequisites guide](https://tauri.app/start/prerequisites/)

---

## Начало работы

```bash
npm install
npm run dev      # debug-запуск с hot reload
npm run build    # → iStickies.app + .dmg в src-tauri/target/release/bundle
```

Заметки и настройки лежат в `~/Library/Application Support/com.dmytrofiliurskyi.stickies` (на Linux `~/.local/share/stickies`)

---

## Структура проекта

```
stickies/
├── src/
│   ├── index.html / main.js   - окно менеджера: список, цвета, настройки
│   ├── note.html / note.js    - окно заметки: только показ, drag + resize
│   ├── styles.css             - стили бумаги, кнопки и markdown
│   └── vendor/marked.min.js   - офлайн markdown-рендерер
├── src-tauri/
│   ├── src/lib.rs             - команды: заметки, bounds, vim, tray, окна
│   ├── capabilities/          - наборы разрешений Tauri
│   ├── tauri.conf.json        - окна, bundle и метаданные приложения
│   └── icons/                 - иконки приложения и трея
└── package.json               - tauri / dev / build скрипты
```

---

## Известные ограничения

- **Wayland** отдает always-on-bottom композитору, заметки могут вести себя как обычные окна
- **Windows**-поддержка vim-флоу пока заглушка, сначала macOS и Linux
- **Первый запуск** ad-hoc подписан, сними quarantine через `xattr -cr` (см. Установка)
- **Markdown-картинки** из сети не грузятся, рендерер сознательно офлайн

---

## Лицензия

MIT © Dmytro Filiurskyi
