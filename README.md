<div align="center">

# Modern TODO
### Simple Minimal Task Manager

Clean. Focused. Distraction-free. Built with Rust.

</div>

---

## Overview

<img width="1917" height="875" alt="image" src="https://github.com/user-attachments/assets/9bb8014b-bc36-444f-ab94-3aee9ab86359" />

Modern TODO is a simple and minimal task manager built using Rust and GTK4.

It is designed for clarity and focus.  
No unnecessary features. No visual noise.  
Just a clean interface for managing daily tasks efficiently.

The application follows a vertical layout optimized for straightforward task entry and organization.

---

## Features

- Simple and intuitive interface  
- Calendar-based task organization  
- Tasks filtered by selected date  
- Clean minimal design  
- Lightweight and fast  
- Local JSON-based storage  
- No external dependencies or accounts  

---

## Design Philosophy

Modern TODO is built around simplicity:

- Minimal visual elements  
- Clear structure  
- Focused task workflow  
- No clutter or distractions  
- Productivity-first layout  

The goal is to provide a calm and efficient task management experience.

---

## Tech Stack

| Component | Description |
|------------|-------------|
| Rust | Core application logic |
| GTK4 | Native Linux UI |
| GLib | Application runtime |
| Serde | JSON persistence |

---

# Installation

You can install Modern TODO in two ways:

---

## Option 1 — Automatic Installation (Recommended)

This method installs Rust (if missing), builds the project, installs the binary system-wide, and creates a launcher entry automatically.

### Run Installer

```bash
chmod +x install.sh
./install.sh
```

The script will:

- Install Rust via rustup (if not installed)
- Build release binary
- Install to `/usr/local/bin/todolist`
- Create desktop entry
- Install icon (if available)

After installation, you can launch:

```
todolist
```

Or open it from your application launcher / Rofi.

---

## Option 2 — Manual Installation

### 1. Clone Repository

```bash
git clone https://github.com/c1ph3r-1337/To-Do-List.git
cd rust_todo
```

### 2. Build

```bash
cargo build --release
```

### 3. Run

```bash
cargo run
```

Or run the compiled binary:

```bash
./target/release/rust_todo
```

---

### Optional: Install System-wide Manually

```bash
sudo cp target/release/rust_todo /usr/local/bin/todolist
```

---

## Desktop Entry (Manual Setup)

If installing manually, create:

```
~/.local/share/applications/todolist.desktop
```

Add:

```ini
[Desktop Entry]
Name=Modern TODO
Comment=Simple Minimal Task Manager
Exec=/usr/local/bin/todolist
Icon=todolist
Terminal=false
Type=Application
Categories=Utility;Productivity;
StartupWMClass=com.todo.gtk
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
├── install.sh
└── README.md
```

---

## Data Storage

Tasks are stored locally in:

```
~/.rust_todo.json
```

All data remains on your machine.

---

## Architecture Overview

- GTK4 handles rendering  
- State managed via `Rc<RefCell<Vec<Task>>>`  
- Persistent storage via Serde JSON  
- Lightweight and self-contained  

---

## Future Improvements

- Drag-and-drop reordering  
- Task categories  
- Search functionality  
- Improved keyboard navigation  

---

## License

MIT License

---

<div align="center">

Built with Rust for simplicity and focus.

</div>