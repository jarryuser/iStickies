const { invoke } = window.__TAURI__.core;

const listEl = document.getElementById("list");
const terminalSel = document.getElementById("terminal");
const terminalCustomRow = document.getElementById("terminal-custom-row");
const terminalCustom = document.getElementById("terminal-custom");
const editorInput = document.getElementById("editor");
const editModeSel = document.getElementById("edit-mode");
const settingsStatus = document.getElementById("settings-status");
let editMode = "vim";
const errEl = document.createElement("div");
errEl.className = "error";
errEl.style.display = "none";
listEl.before(errEl);

function showError(e) {
  console.error(e);
  errEl.style.display = "block";
  errEl.textContent = typeof e === "string" ? e : (e.message || JSON.stringify(e));
  setTimeout(() => { errEl.style.display = "none"; }, 5000);
}

async function openNoteWindow(note) {
  try {
    await invoke("show_note", { id: note.id });
  } catch (e) {
    try {
      const { WebviewWindow } = window.__TAURI__.webviewWindow;
      const label = `note-${note.id}`;
      const existing = await WebviewWindow.getByLabel(label);
      if (existing) {
        await existing.show();
        await existing.setFocus();
        return;
      }
      const win = new WebviewWindow(label, {
        url: `note.html?id=${note.id}`,
        title: "Sticky",
        width: Math.round(note.width || 280),
        height: Math.round(note.height || 280),
        x: Math.round(note.x ?? 120),
        y: Math.round(note.y ?? 120),
        decorations: false,
        resizable: true,
        alwaysOnTop: false,
        alwaysOnBottom: true,
        skipTaskbar: true,
      });
      win.once("tauri://error", showError);
    } catch (fallbackErr) {
      showError(fallbackErr);
    }
  }
}

function preview(text) {
  const t = (text || "").trim();
  if (!t) return "(empty)";
  return t.length > 80 ? t.slice(0, 80) + "..." : t;
}

const COLORS = ["#fff7a8", "#ffd1dc", "#cfe8ff", "#d8f5c8", "#e5d4ff"];

async function refresh() {
  let notes = [];
  try {
    notes = await invoke("list_notes");
  } catch (e) {
    showError(e);
    return;
  }
  listEl.innerHTML = "";
  if (!notes.length) {
    listEl.innerHTML = `<div class="empty">No notes yet. Hit + New.</div>`;
    return;
  }
  for (const n of notes) {
    const row = document.createElement("div");
    row.className = "row-note";
    row.style.borderLeft = `4px solid ${n.color || "#fff7a8"}`;
    row.innerHTML = `
      <div class="row-text">${escapeHtml(preview(n.content))}</div>
      <div class="row-colors">${COLORS.map((c) => `<span class="swatch${c === n.color ? " active" : ""}" data-color="${c}" style="background:${c}"></span>`).join("")}</div>
      <div class="row-btns">
        <button data-act="open">show</button>
        <button data-act="vim">vim</button>
        <button data-act="hide">hide</button>
        <button data-act="del" class="danger">x</button>
      </div>`;
    row.querySelector('[data-act="open"]').onclick = () => openNoteWindow(n);
    const editBtn = row.querySelector('[data-act="vim"]');
    editBtn.textContent = editMode === "builtin" ? "edit" : "vim";
    editBtn.onclick = async () => {
      try {
        if (editMode === "builtin") {
          await invoke("request_edit", { id: n.id });
        } else {
          await openNoteWindow(n);
          await invoke("edit_in_vim", { id: n.id });
        }
      } catch (e) {
        showError(e);
      }
    };
    row.querySelector('[data-act="hide"]').onclick = async () => {
      try {
        await invoke("hide_note", { id: n.id });
      } catch (e) {
        showError(e);
      }
    };
    row.querySelectorAll(".swatch").forEach((s) => {
      s.onclick = async () => {
        try {
          await invoke("set_color", { id: n.id, color: s.dataset.color });
          refresh();
        } catch (e) {
          showError(e);
        }
      };
    });
    row.querySelector('[data-act="del"]').onclick = async () => {
      try {
        await invoke("delete_note", { id: n.id });
        refresh();
      } catch (e) {
        showError(e);
      }
    };
    listEl.appendChild(row);
  }
}

function escapeHtml(s) {
  return s.replace(/[&<>"']/g, (c) => ({
    "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;",
  }[c]));
}

async function loadSettings() {
  try {
    const [settings, terminals] = await Promise.all([
      invoke("get_settings"),
      invoke("list_terminals").catch(() => []),
    ]);
    const detected = [...new Set([...(terminals || [])].filter(Boolean))];
    terminalSel.innerHTML = "";
    for (const t of detected) {
      const opt = document.createElement("option");
      opt.value = t;
      opt.textContent = t;
      terminalSel.appendChild(opt);
    }
    const customOpt = document.createElement("option");
    customOpt.value = "__custom";
    customOpt.textContent = "Custom...";
    terminalSel.appendChild(customOpt);
    if (settings.terminal && detected.includes(settings.terminal)) {
      terminalSel.value = settings.terminal;
    } else if (settings.terminal) {
      terminalSel.value = "__custom";
      terminalCustom.value = settings.terminal;
    } else {
      terminalSel.value = detected[0] || "__custom";
    }
    syncCustomRow();
    terminalSel.onchange = syncCustomRow;
    editorInput.value = settings.editor || "";
    editMode = settings.edit_mode === "builtin" ? "builtin" : "vim";
    editModeSel.value = editMode;
    document.getElementById("show-pin").checked = settings.show_pin !== false;
    document.getElementById("pin-color").value = settings.pin_color || "#d21f1f";
  } catch (e) {
    showError(e);
  }
}

function syncCustomRow() {
  terminalCustomRow.style.display = terminalSel.value === "__custom" ? "" : "none";
}

function chosenTerminal() {
  if (terminalSel.value === "__custom") return terminalCustom.value.trim();
  return terminalSel.value || "";
}

document.getElementById("save-settings").onclick = async () => {
  try {
    await invoke("save_settings", {
      terminal: chosenTerminal(),
      editor: editorInput.value || "",
      showPin: document.getElementById("show-pin").checked,
      pinColor: document.getElementById("pin-color").value || "#d21f1f",
      editMode: editModeSel.value === "builtin" ? "builtin" : "vim",
    });
    editMode = editModeSel.value === "builtin" ? "builtin" : "vim";
    refresh();
    settingsStatus.textContent = "saved";
    setTimeout(() => { settingsStatus.textContent = ""; }, 2000);
  } catch (e) {
    showError(e);
  }
};

document.getElementById("new-note").onclick = async () => {
  try {
    await invoke("create_note", { content: "" });
    refresh();
  } catch (e) {
    showError(e);
  }
};

document.getElementById("show-all").onclick = async () => {
  try {
    await invoke("show_all_notes");
  } catch (e) {
    showError(e);
  }
};

document.getElementById("hide-all").onclick = async () => {
  try {
    await invoke("hide_all_notes");
  } catch (e) {
    showError(e);
  }
};

window.addEventListener("DOMContentLoaded", async () => {
  await loadSettings();
  await refresh();
});
