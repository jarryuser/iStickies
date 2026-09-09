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

## Початок роботи

```bash
npm install
npm run dev      # debug-запуск з hot reload
npm run build    # → stickies.app + .dmg у src-tauri/target/release/bundle
```

Потрібні Rust 1.97+, Node 18+ та Xcode Command Line Tools на macOS. Нотатки й налаштування лежать у `~/Library/Application Support/com.dmytrofiliurskyi.stickies` (на Linux `~/.local/share/stickies`).

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

## Відомі обмеження

- **Wayland** віддає always-on-bottom композитору, нотатки можуть поводитись як звичайні вікна
- **Windows**-підтримка vim-флоу поки заглушка, спочатку macOS і Linux
- **Перший запуск** непідписаний, відкрий правим кліком → Open або зніми quarantine-флаг
- **Markdown-зображення** з мережі не вантажаться, рендерер свідомо офлайн

---

## Ліцензія

MIT © Dmytro Filiurskyi
