#!/usr/bin/env bash

set -e

APP_NAME="todolist"
BINARY_NAME="rust_todo"
INSTALL_PATH="/usr/local/bin"
DESKTOP_PATH="$HOME/.local/share/applications"
ICON_PATH="$HOME/.local/share/icons/hicolor/256x256/apps"

echo "Building release binary..."
cargo build --release

echo "Installing binary..."
sudo cp target/release/$BINARY_NAME $INSTALL_PATH/$APP_NAME
sudo chmod +x $INSTALL_PATH/$APP_NAME

echo "Creating desktop entry..."
mkdir -p $DESKTOP_PATH

cat > $DESKTOP_PATH/$APP_NAME.desktop <<EOF
[Desktop Entry]
Name=Modern TODO
Comment=Simple Minimal Task Manager
Exec=$INSTALL_PATH/$APP_NAME
Icon=$APP_NAME
Terminal=false
Type=Application
Categories=Utility;Productivity;
StartupWMClass=com.todo.gtk
EOF

echo "Installing icon (if exists)..."
if [ -f "assets/icon.png" ]; then
    mkdir -p $ICON_PATH
    cp assets/icon.png $ICON_PATH/$APP_NAME.png
    gtk-update-icon-cache ~/.local/share/icons/hicolor || true
fi

echo "Updating desktop database..."
update-desktop-database $DESKTOP_PATH || true

echo "Installation complete."
echo "You can now launch '$APP_NAME' from Rofi or your application launcher."
