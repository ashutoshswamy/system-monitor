#!/bin/sh
# Builds dist/System Monitor.app and dist/System-Monitor-<version>-macos-<arch>.dmg from the release binary.
set -eu
cd "$(dirname "$0")/.."
VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)
ARCH=$(uname -m)
BIN=target/release/system-monitor
cargo build --release

APP="dist/System Monitor.app"
rm -rf dist && mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
cp "$BIN" "$APP/Contents/MacOS/System Monitor"

# Icon from assets/icon.svg at every size macOS asks for.
ICONSET=$(mktemp -d)/icon.iconset
mkdir -p "$ICONSET"
for s in 16 32 128 256 512; do
    swift scripts/svg2png.swift assets/icon.svg "$ICONSET/icon_${s}x${s}.png" $s
    swift scripts/svg2png.swift assets/icon.svg "$ICONSET/icon_${s}x${s}@2x.png" $((s * 2))
done
iconutil -c icns "$ICONSET" -o "$APP/Contents/Resources/icon.icns"
rm -rf "$(dirname "$ICONSET")"

cat > "$APP/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
  <key>CFBundleName</key><string>System Monitor</string>
  <key>CFBundleDisplayName</key><string>System Monitor</string>
  <key>CFBundleIdentifier</key><string>dev.system-monitor.app</string>
  <key>CFBundleIconFile</key><string>icon</string>
  <key>CFBundleExecutable</key><string>System Monitor</string>
  <key>CFBundleVersion</key><string>$VERSION</string>
  <key>CFBundleShortVersionString</key><string>$VERSION</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>LSMinimumSystemVersion</key><string>11.0</string>
  <key>NSHighResolutionCapable</key><true/>
  <key>NSSupportsAutomaticGraphicsSwitching</key><true/>
</dict></plist>
EOF
# Ad-hoc signature: required for arm64 to run; not a Developer ID signature.
codesign --force --deep -s - "$APP"

DMG="dist/System-Monitor-$VERSION-macos-$ARCH.dmg"
STAGE=$(mktemp -d)
cp -R "$APP" "$STAGE/"
ln -s /Applications "$STAGE/Applications"
hdiutil create -volname "System Monitor" -srcfolder "$STAGE" -ov -format UDZO "$DMG" >/dev/null
rm -rf "$STAGE"
echo "$DMG"
