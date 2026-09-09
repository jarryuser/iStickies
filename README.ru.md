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

## Начало работы

```bash
npm install
npm run dev      # debug-запуск с hot reload
npm run build    # → stickies.app + .dmg в src-tauri/target/release/bundle
```

Нужны Rust 1.97+, Node 18+ и Xcode Command Line Tools на macOS. Заметки и настройки лежат в `~/Library/Application Support/com.dmytrofiliurskyi.stickies` (на Linux `~/.local/share/stickies`).

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
- **Первый запуск** неподписан, открой правым кликом → Open или сними quarantine-флаг
- **Markdown-картинки** из сети не грузятся, рендерер сознательно офлайн

---

## Лицензия

MIT © Dmytro Filiurskyi
