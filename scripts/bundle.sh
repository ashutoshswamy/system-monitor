#!/bin/sh
# Build a release .app with the dial icon: target/release/System Monitor.app
set -eu
cd "$(dirname "$0")/.."

APP="target/release/System Monitor.app"
VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)

cargo build --release

ICONSET=target/AppIcon.iconset
rm -rf "$ICONSET" "$APP"
mkdir -p "$ICONSET" "$APP/Contents/MacOS" "$APP/Contents/Resources"
for s in 16 32 128 256 512; do
    swift scripts/svg2png.swift assets/icon.svg "$ICONSET/icon_${s}x${s}.png" $s
    swift scripts/svg2png.swift assets/icon.svg "$ICONSET/icon_${s}x${s}@2x.png" $((s * 2))
done
iconutil -c icns "$ICONSET" -o "$APP/Contents/Resources/AppIcon.icns"

cp target/release/system-monitor "$APP/Contents/MacOS/system-monitor"
cat > "$APP/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key><string>System Monitor</string>
    <key>CFBundleDisplayName</key><string>System Monitor</string>
    <key>CFBundleIdentifier</key><string>dev.local.system-monitor</string>
    <key>CFBundleVersion</key><string>$VERSION</string>
    <key>CFBundleShortVersionString</key><string>$VERSION</string>
    <key>CFBundleExecutable</key><string>system-monitor</string>
    <key>CFBundleIconFile</key><string>AppIcon</string>
    <key>CFBundlePackageType</key><string>APPL</string>
    <key>LSMinimumSystemVersion</key><string>13.0</string>
    <key>NSHighResolutionCapable</key><true/>
</dict>
</plist>
PLIST
# Ad-hoc signature so Apple Silicon will launch the bundle (not notarized).
codesign --force --deep -s - "$APP"
ditto -c -k --keepParent "$APP" "target/release/System-Monitor-$VERSION-macos-$(uname -m).zip"
echo "Built $APP and zip"
