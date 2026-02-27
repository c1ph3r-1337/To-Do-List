Modern TODO – GTK4 Liquid Glass Task Manager

Modern TODO is a vertically optimized task manager built with Rust and GTK4.
It is designed for Wayland compositors such as Hyprland and features a glass-style translucent interface with date-based task management.

Features

Written in Rust (Edition 2024)

GTK4-based native Linux UI

Calendar-based task scheduling

Tasks filtered by selected date

Glass-style translucent interface

Floating vertical layout (420x720)

Hyprland blur integration

Rounded corners and transparency

JSON-based persistent storage

Lightweight and fast

Preview

Add a screenshot inside:

assets/screenshot.png
Tech Stack

Rust

GTK4

GLib

Serde (JSON serialization)

Hyprland (blur and window rules)

Installation
Clone Repository
git clone https://github.com/YOUR_USERNAME/rust_todo.git
cd rust_todo
Build
cargo build --release
Run
cargo run

Or run compiled binary:

./target/release/rust_todo
Install System-wide (Optional)
sudo cp target/release/rust_todo /usr/local/bin/todolist
Desktop Entry (Rofi / Launcher)

Create:

~/.local/share/applications/todolist.desktop

Add:

[Desktop Entry]
Name=Modern TODO
Comment=Liquid Glass Task Manager
Exec=/usr/local/bin/todolist
Icon=todolist
Terminal=false
Type=Application
Categories=Utility;Productivity;
StartupWMClass=com.todo.gtk
Hyprland Configuration (Glass Setup)

Add to hyprland.conf:

windowrule {
    name = todo_app
    match:class = ^(com.todo.gtk)$
    float = true
    center = true
    size = 420 720
    opacity = 0.80 0.80 1
}

Enable blur:

decoration {
    rounding = 24

    blur {
        enabled = true
        size = 18
        passes = 4
        ignore_opacity = false
    }
}

Reload Hyprland:

hyprctl reload
Project Structure
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
Data Storage

Tasks are stored locally in JSON format:

~/.rust_todo.json
Architecture Overview

GTK4 handles rendering

Tasks stored in Vec<Task>

State managed using Rc<RefCell<>>

Serialization handled via Serde

Blur handled at compositor level by Hyprland

Future Improvements

Drag and drop reordering

Task categories

System notifications

SQLite backend

System tray integration

UI animations and transitions

License

MIT License

Author

Built with Rust and GTK4 by c1ph3r-1337