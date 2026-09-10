use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Note {
    id: String,
    content: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    color: String,
    updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Settings {
    terminal: String,
    editor: String,
    #[serde(default = "pin_default")]
    show_pin: bool,
    #[serde(default = "pin_color_default")]
    pin_color: String,
    #[serde(default = "edit_mode_default")]
    edit_mode: String,
    #[serde(default)]
    data_dir: String,
}

fn pin_default() -> bool {
    true
}

fn edit_mode_default() -> String {
    "vim".to_string()
}

fn pin_color_default() -> String {
    "#d21f1f".to_string()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            #[cfg(target_os = "macos")]
            terminal: "Terminal".to_string(),
            #[cfg(not(target_os = "macos"))]
            terminal: String::new(),
            editor: String::new(),
            show_pin: true,
            pin_color: pin_color_default(),
            edit_mode: edit_mode_default(),
            data_dir: String::new(),
        }
    }
}

fn settings_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("settings.json"))
}

fn load_settings(app: &AppHandle) -> Settings {
    let path = match settings_path(app) {
        Ok(p) => p,
        Err(_) => return Settings::default(),
    };
    let data = fs::read_to_string(path).unwrap_or_default();
    if data.trim().is_empty() {
        return Settings::default();
    }
    serde_json::from_str(&data).unwrap_or_default()
}

fn save_settings_inner(app: &AppHandle, s: &Settings) -> Result<(), String> {
    let path = settings_path(app)?;
    let data = serde_json::to_string_pretty(s).map_err(|e| e.to_string())?;
    fs::write(path, data).map_err(|e| e.to_string())
}

fn expand_dir(raw: &str) -> Result<PathBuf, String> {
    let t = raw.trim();
    if t.is_empty() {
        return Err("empty path".to_string());
    }
    let expanded = if t == "~" || t.starts_with("~/") {
        let home = std::env::var("HOME").map_err(|_| "no HOME".to_string())?;
        format!("{}{}", home, &t[1..])
    } else {
        t.to_string()
    };
    let p = PathBuf::from(expanded);
    if !p.is_absolute() {
        return Err("path must be absolute".to_string());
    }
    Ok(p)
}

// Notes live in the custom dir when set, settings.json
// always stays in the default app data dir.
fn notes_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let custom = load_settings(app).data_dir;
    if custom.trim().is_empty() {
        let dir = app
            .path()
            .app_data_dir()
            .map_err(|e| e.to_string())?;
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        return Ok(dir);
    }
    let dir = expand_dir(&custom)?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let probe = dir.join(".write-test");
    fs::write(&probe, "ok").map_err(|e| e.to_string())?;
    let _ = fs::remove_file(&probe);
    Ok(dir)
}

fn store_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(notes_dir(app)?.join("notes.json"))
}

fn edit_path(app: &AppHandle, id: &str) -> Result<PathBuf, String> {
    Ok(notes_dir(app)?.join(format!("edit-{}.md", id)))
}

fn load_all(app: &AppHandle) -> Vec<Note> {
    let path = match store_path(app) {
        Ok(p) => p,
        Err(_) => return vec![],
    };
    let data = fs::read_to_string(path).unwrap_or_default();
    if data.trim().is_empty() {
        return vec![];
    }
    serde_json::from_str(&data).unwrap_or_default()
}

fn save_all(app: &AppHandle, notes: &[Note]) -> Result<(), String> {
    let path = store_path(app)?;
    let data = serde_json::to_string_pretty(notes).map_err(|e| e.to_string())?;
    fs::write(path, data).map_err(|e| e.to_string())
}

fn shell_escape(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn now_ts() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn applescript_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn is_executable(p: &std::path::Path) -> bool {
    let m = match std::fs::metadata(p) {
        Ok(m) => m,
        Err(_) => return false,
    };
    if !m.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        m.permissions().mode() & 0o111 != 0
    }
    #[cfg(not(unix))]
    {
        true
    }
}

// Absolute path of a binary, GUI apps get a bare PATH
// without brew and friends, so plain names may not run.
fn find_exe(name: &str) -> Option<String> {
    if name.contains('/') {
        return is_executable(std::path::Path::new(name)).then(|| name.to_string());
    }
    let mut dirs: Vec<String> = std::env::var("PATH")
        .unwrap_or_default()
        .split(':')
        .map(|s| s.to_string())
        .collect();
    for extra in ["/opt/homebrew/bin", "/usr/local/bin", "/opt/local/bin"] {
        if !dirs.iter().any(|d| d == extra) {
            dirs.push(extra.to_string());
        }
    }
    dirs.iter()
        .map(|d| format!("{}/{}", d, name))
        .find(|p| is_executable(std::path::Path::new(p)))
}

// Editor as absolute path with flags kept, first hit wins.
// Terminals like Ghostty run it under login with a bare
// PATH, so a bare `nvim` dies there while `vim` works.
fn resolve_editor(override_editor: &str) -> String {
    let mut cands: Vec<String> = vec![];
    let v = override_editor.trim();
    if !v.is_empty() {
        cands.push(v.to_string());
    }
    for key in ["VISUAL", "EDITOR"] {
        if let Ok(v) = std::env::var(key) {
            let v = v.trim().to_string();
            if !v.is_empty() {
                cands.push(v);
            }
        }
    }
    cands.extend(["nvim", "vim", "vi", "nano"].iter().map(|s| s.to_string()));
    for cand in cands {
        let mut parts = cand.split_whitespace();
        let bin = match parts.next() {
            Some(b) => b,
            None => continue,
        };
        if let Some(abs) = find_exe(bin) {
            let rest: Vec<&str> = parts.collect();
            if rest.is_empty() {
                return abs;
            }
            return format!("{} {}", abs, rest.join(" "));
        }
    }
    "vi".to_string()
}

fn which_candidate(bin: &str) -> bool {
    std::process::Command::new("which")
        .arg(bin)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

// Real executable for a GUI terminal: CLI in PATH first,
// then the binary inside the .app bundle. Needed because
// `open -a --args` drops args for an already running app.
fn gui_exe(cli: &str, app: &str, exe: &str) -> Option<String> {
    if which_candidate(cli) {
        return Some(cli.to_string());
    }
    let home = std::env::var("HOME").unwrap_or_default();
    for dir in [
        "/Applications".to_string(),
        format!("{}/Applications", home),
    ] {
        let p = format!("{}/{}.app/Contents/MacOS/{}", dir, app, exe);
        if std::path::Path::new(&p).exists() {
            return Some(p);
        }
    }
    None
}

fn open_terminal_macos(app_name: &str, shell_cmd: &str) -> Result<(), String> {
    let name = app_name.trim();
    if name.is_empty() || name == "Terminal" {
        let script = format!(
            "tell application \"Terminal\" to do script \"{}\"\ntell application \"Terminal\" to activate",
            applescript_escape(shell_cmd)
        );
        std::process::Command::new("osascript")
            .args(["-e", &script])
            .spawn()
            .map_err(|e| e.to_string())?;
        return Ok(());
    }
    if name == "iTerm2" || name == "iTerm" {
        let script = format!(
            "tell application \"iTerm2\"\ncreate window with default profile command \"{}\"\nactivate\nend tell",
            applescript_escape(shell_cmd)
        );
        std::process::Command::new("osascript")
            .args(["-e", &script])
            .spawn()
            .map_err(|e| e.to_string())?;
        return Ok(());
    }
    // GUI terminals need their own exec flag, otherwise they just
    // open a default shell and the editor never starts.
    let lower = name.to_lowercase();
    let (cli, app, exe, prefix): (&str, &str, &str, Vec<&str>) = match lower.as_str() {
        "ghostty" => ("ghostty", "Ghostty", "ghostty", vec!["-e"]),
        "alacritty" => ("alacritty", "Alacritty", "alacritty", vec!["-e"]),
        "kitty" => ("kitty", "kitty", "kitty", vec![]),
        "wezterm" => ("wezterm", "WezTerm", "wezterm", vec!["start", "--"]),
        "xterm" => ("xterm", "XQuartz", "xterm", vec!["-e"]),
        _ => (name, name, lower.as_str(), vec!["-e"]),
    };
    // Prefer the real executable: `open -a --args` silently drops
    // args when the app is already running, opening a plain shell.
    if let Some(bin) = gui_exe(cli, app, exe) {
        let mut cmd = std::process::Command::new(bin);
        for a in &prefix {
            cmd.arg(a);
        }
        cmd.arg("sh").arg("-c").arg(shell_cmd);
        cmd.spawn().map_err(|e| e.to_string())?;
        return Ok(());
    }
    // Last resort: new app instance, a reused one drops --args.
    let mut cmd = std::process::Command::new("open");
    cmd.args(["-na", app, "--args"]);
    for a in &prefix {
        cmd.arg(a);
    }
    cmd.arg("sh").arg("-c").arg(shell_cmd);
    cmd.spawn().map_err(|e| e.to_string())?;
    Ok(())
}

fn open_in_terminal(editor: &str, file: &PathBuf, terminal_pref: &str) -> Result<(), String> {
    let file_str = file.to_string_lossy().to_string();
    let shell_cmd = format!("{} {}; exit", editor, shell_escape(&file_str));

    #[cfg(target_os = "macos")]
    {
        if terminal_pref.trim().is_empty() {
            return open_terminal_macos("Terminal", &shell_cmd);
        }
        return open_terminal_macos(terminal_pref, &shell_cmd);
    }

    #[cfg(target_os = "windows")]
    {
        let _ = terminal_pref;
        let file_arg = file_str.clone();
        std::process::Command::new("cmd")
            .args(["/C", "start", "", editor, &file_arg])
            .spawn()
            .map_err(|e| e.to_string())?;
        return Ok(());
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        let inner = shell_cmd.clone();
        if !terminal_pref.trim().is_empty() {
            let term = terminal_pref.trim();
            let extra: Vec<&str> = if term.contains("gnome-terminal") {
                vec!["--"]
            } else if term == "kitty" {
                vec![]
            } else {
                vec!["-e"]
            };
            let mut cmd = std::process::Command::new(term);
            for a in extra {
                cmd.arg(a);
            }
            if term == "kitty" {
                cmd.arg(editor).arg(&file_str);
            } else {
                cmd.arg("sh").arg("-c").arg(&inner);
            }
            if cmd.spawn().is_ok() {
                return Ok(());
            }
            return Err(format!("cannot open terminal '{}'", term));
        }
        let terminals: Vec<Vec<&str>> = vec![
            vec!["x-terminal-emulator", "-e"],
            vec!["gnome-terminal", "--"],
            vec!["konsole", "-e"],
            vec!["alacritty", "-e"],
            vec!["kitty"],
            vec!["xfce4-terminal", "-e"],
            vec!["xterm", "-e"],
        ];
        for term in terminals {
            let (bin, args) = term.split_first().unwrap();
            if !which_candidate(bin) {
                continue;
            }
            let mut cmd = std::process::Command::new(bin);
            for a in args {
                cmd.arg(a);
            }
            if *bin == "kitty" {
                cmd.arg(editor).arg(&file_str);
            } else {
                cmd.arg("sh").arg("-c").arg(&inner);
            }
            if cmd.spawn().is_ok() {
                return Ok(());
            }
        }
        Err("no terminal emulator found".to_string())
    }
}

#[tauri::command]
fn show_note(app: AppHandle, id: String) -> Result<(), String> {
    let label = format!("note-{}", id);
    if let Some(w) = app.get_webview_window(&label) {
        w.show().map_err(|e| e.to_string())?;
        w.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }
    let notes = load_all(&app);
    let note = notes
        .into_iter()
        .find(|n| n.id == id)
        .ok_or("note not found")?;
    let url = WebviewUrl::App(format!("note.html?id={}", note.id).into());
    let window = WebviewWindowBuilder::new(&app, label, url)
        .title("Sticky")
        .inner_size(note.width, note.height)
        .position(note.x, note.y)
        .decorations(false)
        .always_on_top(false)
        .always_on_bottom(true)
        .skip_taskbar(true)
        .resizable(true)
        .shadow(true)
        .build()
        .map_err(|e| e.to_string())?;
    pin_to_desktop(&window);
    Ok(())
}

// Keeps note windows visible on Show Desktop / Expose.
// Tauri exposes only CanJoinAllSpaces, Stationary needs AppKit directly.
#[cfg(target_os = "macos")]
fn pin_to_desktop(window: &tauri::WebviewWindow) {
    use objc2_app_kit::{NSWindow, NSWindowCollectionBehavior};
    let ptr = match window.ns_window() {
        Ok(p) => p as *mut NSWindow,
        Err(_) => return,
    };
    if ptr.is_null() {
        return;
    }
    let win: &NSWindow = unsafe { &*ptr };
    let cur = win.collectionBehavior();
    win.setCollectionBehavior(
        cur | NSWindowCollectionBehavior::Stationary
            | NSWindowCollectionBehavior::CanJoinAllSpaces,
    );
}

#[cfg(not(target_os = "macos"))]
fn pin_to_desktop(_window: &tauri::WebviewWindow) {}

#[tauri::command]
fn show_all_notes(app: AppHandle) -> Result<(), String> {
    let ids: Vec<String> = load_all(&app).into_iter().map(|n| n.id).collect();
    for id in ids {
        show_note(app.clone(), id)?;
    }
    Ok(())
}

#[tauri::command]
fn list_notes(app: AppHandle) -> Vec<Note> {
    load_all(&app)
}

#[tauri::command]
fn get_note(app: AppHandle, id: String) -> Option<Note> {
    load_all(&app).into_iter().find(|n| n.id == id)
}

#[tauri::command]
fn create_note(app: AppHandle, content: Option<String>) -> Result<Note, String> {
    let mut notes = load_all(&app);
    let offset = (notes.len() as f64 % 10.0) * 28.0;
    let note = Note {
        id: uuid::Uuid::new_v4().to_string(),
        content: content.unwrap_or_default(),
        x: 120.0 + offset,
        y: 120.0 + offset,
        width: 280.0,
        height: 280.0,
        color: "#fff7a8".to_string(),
        updated_at: now_ts(),
    };
    notes.push(note.clone());
    save_all(&app, &notes)?;
    let _ = show_note(app, note.id.clone());
    Ok(note)
}

#[tauri::command]
fn delete_note(app: AppHandle, id: String) -> Result<(), String> {
    let notes: Vec<Note> = load_all(&app).into_iter().filter(|n| n.id != id).collect();
    save_all(&app, &notes)?;
    if let Some(w) = app.get_webview_window(&format!("note-{}", id)) {
        let _ = w.close();
    }
    Ok(())
}

#[tauri::command]
fn update_bounds(
    app: AppHandle,
    id: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) -> Result<(), String> {
    let mut notes = load_all(&app);
    if let Some(n) = notes.iter_mut().find(|n| n.id == id) {
        n.x = x;
        n.y = y;
        n.width = width;
        n.height = height;
        n.updated_at = now_ts();
        save_all(&app, &notes)?;
        Ok(())
    } else {
        Err("note not found".to_string())
    }
}

#[tauri::command]
fn get_settings(app: AppHandle) -> Settings {
    load_settings(&app)
}

#[tauri::command]
fn save_settings(
    app: AppHandle,
    terminal: String,
    editor: String,
    show_pin: bool,
    pin_color: String,
    edit_mode: String,
) -> Result<Settings, String> {
    let c = pin_color.trim();
    let ok = (c.len() == 7 || c.len() == 4)
        && c.starts_with('#')
        && c[1..].chars().all(|ch| ch.is_ascii_hexdigit());
    if !ok {
        return Err("bad pin color, use #rgb or #rrggbb".to_string());
    }
    let mode = edit_mode.trim().to_lowercase();
    if mode != "vim" && mode != "builtin" {
        return Err("edit mode is vim or builtin".to_string());
    }
    let s = Settings {
        terminal,
        editor,
        show_pin,
        pin_color: c.to_string(),
        edit_mode: mode,
        data_dir: load_settings(&app).data_dir,
    };
    save_settings_inner(&app, &s)?;
    Ok(s)
}

#[tauri::command]
fn set_data_dir(app: AppHandle, path: String) -> Result<Settings, String> {
    let raw = path.trim().to_string();
    let target: Option<PathBuf> = if raw.is_empty() {
        None
    } else {
        let dir = expand_dir(&raw)?;
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let probe = dir.join(".write-test");
        fs::write(&probe, "ok").map_err(|e| e.to_string())?;
        let _ = fs::remove_file(&probe);
        Some(dir)
    };

    let current = load_all(&app);
    let current_file = store_path(&app).ok();

    if let Some(dir) = target {
        // merge with notes already living there, ours win on conflict
        let mut merged: Vec<Note> = fs::read_to_string(dir.join("notes.json"))
            .ok()
            .and_then(|d| serde_json::from_str(&d).ok())
            .unwrap_or_default();
        for n in &current {
            if let Some(old) = merged.iter_mut().find(|m| m.id == n.id) {
                *old = n.clone();
            } else {
                merged.push(n.clone());
            }
        }
        let data = serde_json::to_string_pretty(&merged).map_err(|e| e.to_string())?;
        fs::write(dir.join("notes.json"), data).map_err(|e| e.to_string())?;
        // move pending vim temp files along
        if let Ok(entries) = fs::read_dir(notes_dir(&app).unwrap_or_default()) {
            for e in entries.flatten() {
                let name = e.file_name().to_string_lossy().to_string();
                if name.starts_with("edit-") && name.ends_with(".md") {
                    let _ = fs::rename(e.path(), dir.join(&name));
                }
            }
        }
        if let Some(old) = current_file {
            if old.parent() != Some(dir.as_path()) {
                let _ = fs::remove_file(old);
            }
        }
    } else if let Some(old) = current_file {
        // moving back to default, keep whatever is already there plus ours
        let def = app
            .path()
            .app_data_dir()
            .map_err(|e| e.to_string())?;
        fs::create_dir_all(&def).map_err(|e| e.to_string())?;
        if old.parent() != Some(def.as_path()) {
            let mut merged: Vec<Note> = fs::read_to_string(def.join("notes.json"))
                .ok()
                .and_then(|d| serde_json::from_str(&d).ok())
                .unwrap_or_default();
            for n in &current {
                if let Some(o) = merged.iter_mut().find(|m| m.id == n.id) {
                    *o = n.clone();
                } else {
                    merged.push(n.clone());
                }
            }
            let data = serde_json::to_string_pretty(&merged).map_err(|e| e.to_string())?;
            fs::write(def.join("notes.json"), data).map_err(|e| e.to_string())?;
            let _ = fs::remove_file(old);
        }
    }

    let mut s = load_settings(&app);
    s.data_dir = raw;
    save_settings_inner(&app, &s)?;
    Ok(s)
}

#[tauri::command]
fn list_terminals() -> Vec<String> {
    let mut found = vec![];
    #[cfg(target_os = "macos")]
    {
        // display name, app bundles, CLI binaries
        let cands: &[(&str, &[&str], &[&str])] = &[
            ("Terminal", &["Terminal.app"], &[]),
            ("iTerm2", &["iTerm.app"], &["iterm2"]),
            ("Alacritty", &["Alacritty.app"], &["alacritty"]),
            ("kitty", &["kitty.app"], &["kitty"]),
            ("WezTerm", &["WezTerm.app"], &["wezterm"]),
            ("Ghostty", &["Ghostty.app"], &["ghostty"]),
        ];
        let home = std::env::var("HOME").unwrap_or_default();
        let dirs = [
            "/Applications".to_string(),
            format!("{}/Applications", home),
            "/System/Applications".to_string(),
            "/System/Applications/Utilities".to_string(),
        ];
        for (name, apps, bins) in cands {
            let app_hit = apps.iter().any(|a| {
                dirs.iter()
                    .any(|d| std::path::Path::new(&format!("{}/{}", d, a)).exists())
            });
            let bin_hit = bins.iter().any(|b| which_candidate(b));
            if app_hit || bin_hit {
                found.push(name.to_string());
            }
        }
        if found.is_empty() {
            found.push("Terminal".to_string());
        }
        return found;
    }
    #[cfg(target_os = "windows")]
    {
        return vec!["Windows Terminal".to_string(), "cmd".to_string()];
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        for bin in [
            "gnome-terminal",
            "konsole",
            "alacritty",
            "kitty",
            "xfce4-terminal",
            "xterm",
            "wezterm",
        ] {
            if which_candidate(bin) {
                found.push(bin.to_string());
            }
        }
        return found;
    }
}

#[tauri::command]
fn hide_note(app: AppHandle, id: String) -> Result<(), String> {
    if let Some(w) = app.get_webview_window(&format!("note-{}", id)) {
        w.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn hide_all_notes(app: AppHandle) -> Result<(), String> {
    for note in load_all(&app) {
        if let Some(w) = app.get_webview_window(&format!("note-{}", note.id)) {
            let _ = w.hide();
        }
    }
    Ok(())
}

#[tauri::command]
fn edit_in_vim(app: AppHandle, id: String) -> Result<String, String> {
    let notes = load_all(&app);
    let note = notes
        .into_iter()
        .find(|n| n.id == id)
        .ok_or("note not found")?;

    let path = edit_path(&app, &id)?;
    fs::write(&path, &note.content).map_err(|e| e.to_string())?;
    let before = fs::metadata(&path)
        .and_then(|m| m.modified())
        .unwrap_or(std::time::SystemTime::UNIX_EPOCH);

    let editor = {
        let s = load_settings(&app);
        resolve_editor(&s.editor)
    };
    let terminal = load_settings(&app).terminal;
    open_in_terminal(&editor, &path, &terminal)?;

    // Watch the temp file until the session ends, every save
    // reloads content into the store and notifies open windows.
    let handle = app.clone();
    let watch_path = path.clone();
    let note_id = id.clone();
    std::thread::spawn(move || {
        let mut last = before;
        for _ in 0..3600 {
            std::thread::sleep(Duration::from_millis(500));
            let modified = fs::metadata(&watch_path)
                .and_then(|m| m.modified())
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
            if modified > last {
                last = modified;
                // vim can touch the file twice per save, let it settle
                std::thread::sleep(Duration::from_millis(300));
                if let Ok(content) = fs::read_to_string(&watch_path) {
                    let mut notes = load_all(&handle);
                    if let Some(n) = notes.iter_mut().find(|n| n.id == note_id) {
                        if n.content == content {
                            continue;
                        }
                        n.content = content;
                        n.updated_at = now_ts();
                        let _ = save_all(&handle, &notes);
                        if let Some(updated) =
                            notes.into_iter().find(|n| n.id == note_id)
                        {
                            let _ = handle.emit("note-updated", updated);
                        }
                    }
                }
            }
        }
    });

    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
fn set_color(app: AppHandle, id: String, color: String) -> Result<Note, String> {
    let c = color.trim();
    let ok = (c.len() == 7 || c.len() == 4)
        && c.starts_with('#')
        && c[1..].chars().all(|ch| ch.is_ascii_hexdigit());
    if !ok {
        return Err("bad color, use #rgb or #rrggbb".to_string());
    }
    let mut notes = load_all(&app);
    let note = notes
        .iter_mut()
        .find(|n| n.id == id)
        .ok_or("note not found")?;
    note.color = c.to_string();
    note.updated_at = now_ts();
    let updated = note.clone();
    save_all(&app, &notes)?;
    let _ = app.emit("note-updated", updated.clone());
    Ok(updated)
}

#[tauri::command]
fn update_content(app: AppHandle, id: String, content: String) -> Result<Note, String> {
    let mut notes = load_all(&app);
    let note = notes
        .iter_mut()
        .find(|n| n.id == id)
        .ok_or("note not found")?;
    note.content = content;
    note.updated_at = now_ts();
    let updated = note.clone();
    save_all(&app, &notes)?;
    let _ = app.emit("note-updated", updated.clone());
    Ok(updated)
}

// Asks an open note window to start inline editing.
#[tauri::command]
fn request_edit(app: AppHandle, id: String) -> Result<(), String> {
    show_note(app.clone(), id.clone())?;
    let _ = app.emit("note-edit-requested", id);
    Ok(())
}

fn show_manager(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.set_focus();
        return;
    }
    let _ = WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
        .title("iStickies")
        .inner_size(380.0, 620.0)
        .build();
}

// Red cross hides the manager instead of closing it,
// so the tray icon keeps working and the app stays alive.
fn keep_manager_alive(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let win = w.clone();
        w.on_window_event(move |event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = win.hide();
            }
        });
    }
}

fn build_tray(app: &AppHandle) -> Result<(), String> {
    use tauri::{
        image::Image,
        menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem},
        tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    };
    let icon =
        Image::from_bytes(include_bytes!("../icons/32x32.png")).map_err(|e| e.to_string())?;
    let open = MenuItemBuilder::with_id("manager", "Open iStickies")
        .build(app)
        .map_err(|e| e.to_string())?;
    let new = MenuItemBuilder::with_id("new", "New note")
        .build(app)
        .map_err(|e| e.to_string())?;
    let show_all = MenuItemBuilder::with_id("show-all", "Show all notes")
        .build(app)
        .map_err(|e| e.to_string())?;
    let hide_all = MenuItemBuilder::with_id("hide-all", "Hide all notes")
        .build(app)
        .map_err(|e| e.to_string())?;
    let quit =
        PredefinedMenuItem::quit(app, Some("Quit iStickies")).map_err(|e| e.to_string())?;
    let sep1 = PredefinedMenuItem::separator(app).map_err(|e| e.to_string())?;
    let sep2 = PredefinedMenuItem::separator(app).map_err(|e| e.to_string())?;
    let menu = MenuBuilder::new(app)
        .items(&[&open, &new, &sep1, &show_all, &hide_all, &sep2, &quit])
        .build()
        .map_err(|e| e.to_string())?;
    TrayIconBuilder::new()
        .icon(icon)
        .tooltip("iStickies")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "manager" => {
                show_manager(app);
            }
            "new" => {
                if let Ok(note) = create_note(app.clone(), None) {
                    let _ = show_note(app.clone(), note.id);
                }
            }
            "show-all" => {
                let _ = show_all_notes(app.clone());
            }
            "hide-all" => {
                let _ = hide_all_notes(app.clone());
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button,
                button_state,
                ..
            } = event
            {
                if button == MouseButton::Left && button_state == MouseButtonState::Up {
                    let app = tray.app_handle();
                    if let Some(w) = app.get_webview_window("main") {
                        if w.is_visible().unwrap_or(false) {
                            let _ = w.hide();
                        } else {
                            show_manager(app);
                        }
                    } else {
                        show_manager(app);
                    }
                }
            }
        })
        .build(app)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // no Dock icon, lives in the menu bar instead
            #[cfg(target_os = "macos")]
            let _ = app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            if let Err(e) = build_tray(app.handle()) {
                eprintln!("tray init failed: {}", e);
            }
            keep_manager_alive(app.handle());
            // notes are visible right after launch, nothing stays hidden
            let _ = show_all_notes(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_notes,
            get_note,
            create_note,
            delete_note,
            update_bounds,
            edit_in_vim,
            show_note,
            show_all_notes,
            hide_note,
            hide_all_notes,
            get_settings,
            save_settings,
            list_terminals,
            set_color,
            update_content,
            request_edit,
            set_data_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
