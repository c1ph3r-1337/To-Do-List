<div align="center">

# Modern TODO
### GTK4 Liquid Glass Task Manager

Minimal. Vertical. Glass-styled. Built with Rust.

</div>

---

## Overview

<img width="1919" height="859" alt="image" src="https://github.com/user-attachments/assets/ef99ac6c-8d7b-46d5-8e9a-dc6019f0ad21" />



Modern TODO is a vertically optimized task manager built using **Rust + GTK4**.  
It features a translucent glass-style interface and integrates cleanly with Wayland compositors such as **Hyprland**.

The application is designed as a floating vertical panel with date-based task management.

---

## Features

- Native GTK4 desktop application
- Calendar-based task scheduling
- Tasks filtered by selected date
- Glass-style translucent interface
- Rounded corners and compositor blur support
- JSON-based persistent storage
- Lightweight and fast

---

## Tech Stack

| Component | Description |
|------------|-------------|
| Rust | Core application logic |
| GTK4 | Native Linux UI |
| GLib | Application runtime |
| Serde | JSON serialization |
| Hyprland | Blur & floating window integration |

---

## Installation

### Clone Repository

```bash
git clone https://github.com/YOUR_USERNAME/rust_todo.git
cd rust_todo
```

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run
```

Or execute compiled binary:

```bash
./target/release/rust_todo
```

---

## Optional: Install System-wide

```bash
sudo cp target/release/rust_todo /usr/local/bin/todolist
```

---

## Desktop Entry (Launcher / Rofi)

Create:

```
~/.local/share/applications/todolist.desktop
```

Add:

```ini
[Desktop Entry]
Name=Modern TODO
Comment=Glass Task Manager
Exec=/usr/local/bin/todolist
Icon=todolist
Terminal=false
Type=Application
Categories=Utility;Productivity;
StartupWMClass=com.todo.gtk
```

---

## Hyprland Configuration (Glass Setup)

Add to `hyprland.conf`:

```ini
windowrule {
    name = todo_app
    match:class = ^(com.todo.gtk)$
    float = true
    center = true
    size = 420 720
    opacity = 0.80 0.80 1
}
```

Enable blur:

```ini
decoration {
    rounding = 24

    blur {
        enabled = true
        size = 18
        passes = 4
        ignore_opacity = false
    }
}
```

Reload:

```bash
hyprctl reload
```

---

## Project Structure

```
rust_todo/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs
│   ├── task.rs
│   └── storage.rs
├── assets/
│   └── icon.png
└── README.md
```

---

## Data Storage

Tasks are stored locally:

```
~/.rust_todo.json
```

---

## Architecture Overview

- GTK4 handles rendering
- State managed via `Rc<RefCell<Vec<Task>>>`
- Persistent storage via Serde JSON
- Compositor-level blur provided by Hyprland
- Floating vertical layout optimized for productivity

---

## Future Improvements

- Drag-and-drop task ordering
- Task categories
- System notifications
- SQLite backend
- System tray integration
- UI transitions and animations

---

## License

MIT License

---

<div align="center">

Built with Rust and GTK4.

</div>
