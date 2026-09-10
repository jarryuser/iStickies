const { invoke } = window.__TAURI__.core;
const { getCurrentWindow } = window.__TAURI__.window;
const { listen } = window.__TAURI__.event;

const id = new URLSearchParams(location.search).get("id");
const textEl = document.getElementById("text");
const noteEl = document.getElementById("note");
const pinEl = document.querySelector(".pin");
const appWindow = getCurrentWindow();

let editMode = "vim";
let editing = false;
let lastContent = "";

function applyColor(color) {
  const c = color || "#fff7a8";
  noteEl.style.setProperty("--note-bg", c);
  document.body.style.setProperty("--note-bg", c);
}

function render(content) {
  lastContent = content || "";
  const src = lastContent.trim();
  if (!src) {
    textEl.innerHTML = `<span class="empty-note">double click to edit</span>`;
    return;
  }
  if (window.marked && typeof window.marked.parse === "function") {
    textEl.innerHTML = window.marked.parse(src, { breaks: true });
    textEl.querySelectorAll("li").forEach((li) => {
      const first = li.firstElementChild;
      if (first && first.tagName === "INPUT" && first.type === "checkbox") {
        li.classList.add("task");
      }
    });
  } else {
    textEl.textContent = src;
  }
}

function openEditor() {
  if (editing) return;
  editing = true;
  textEl.innerHTML = "";
  const area = document.createElement("textarea");
  area.className = "note-edit";
  area.value = lastContent;
  const hint = document.createElement("div");
  hint.className = "edit-hint";
  hint.textContent = "Ctrl+Enter save · Esc cancel";
  textEl.appendChild(area);
  textEl.appendChild(hint);
  area.focus();
  area.addEventListener("keydown", (e) => {
    if ((e.ctrlKey || e.metaKey) && e.key === "Enter") {
      e.preventDefault();
      closeEditor(true);
    } else if (e.key === "Escape") {
      e.preventDefault();
      closeEditor(false);
    }
  });
}

async function closeEditor(save) {
  if (!editing) return;
  const area = textEl.querySelector("textarea");
  const value = area ? area.value : lastContent;
  editing = false;
  if (save && id) {
    try {
      const updated = await invoke("update_content", { id, content: value });
      render(updated.content);
      return;
    } catch (e) {
      console.error(e);
    }
  }
  render(lastContent);
}

async function load() {
  if (!id || editing) return;
  const [note, settings] = await Promise.all([
    invoke("get_note", { id }),
    invoke("get_settings").catch(() => null),
  ]);
  if (settings) {
    editMode = settings.edit_mode === "builtin" ? "builtin" : "vim";
    if (pinEl) {
      pinEl.style.display = settings.show_pin === false ? "none" : "";
      const pc = settings.pin_color || "#d21f1f";
      pinEl.style.setProperty("--pin-color", pc);
      document.body.style.setProperty("--pin-color", pc);
    }
  }
  if (note) {
    applyColor(note.color);
    render(note.content);
    document.title = `Sticky ${note.id.slice(0, 6)}`;
  }
}

async function saveBounds() {
  if (!id) return;
  try {
    const [pos, size, factor] = await Promise.all([
      appWindow.outerPosition(),
      appWindow.innerSize(),
      appWindow.scaleFactor(),
    ]);
    await invoke("update_bounds", {
      id,
      x: pos.x / factor,
      y: pos.y / factor,
      width: size.width / factor,
      height: size.height / factor,
    });
  } catch (e) {
    console.error(e);
  }
}

let saveTimer;
function scheduleSave() {
  clearTimeout(saveTimer);
  saveTimer = setTimeout(saveBounds, 600);
}

// double click edits: vim in terminal or inline editor, per settings
textEl.ondblclick = async () => {
  if (!id || editing) return;
  try {
    if (editMode === "builtin") {
      openEditor();
    } else {
      await invoke("edit_in_vim", { id });
    }
  } catch (e) {
    console.error(e);
  }
};

// resize grip uses native resize dragging when available
const grip = document.getElementById("grip");
grip.addEventListener("mousedown", async (e) => {
  e.preventDefault();
  try {
    await appWindow.startResizeDragging("SouthEast");
  } catch {
    // fallback handled by mouse move below
  }
});

if (appWindow.onMoved) appWindow.onMoved(scheduleSave);
if (appWindow.onResized) appWindow.onResized(scheduleSave);
setInterval(saveBounds, 5000);
window.addEventListener("beforeunload", () => {
  closeEditor(true);
  saveBounds();
});

if (listen) {
  listen("note-updated", (ev) => {
    if (editing) return;
    if (ev.payload && ev.payload.id === id) {
      applyColor(ev.payload.color);
      render(ev.payload.content);
    }
  });
  listen("note-edit-requested", (ev) => {
    if (ev.payload === id && editMode === "builtin") {
      openEditor();
    }
  });
}

// poll as fallback in case events are missed
setInterval(load, 3000);

window.addEventListener("DOMContentLoaded", load);
