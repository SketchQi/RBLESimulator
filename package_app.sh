#!/bin/bash
set -e
cd "$(dirname "$0")"
cargo build
APP=target/debug/RBLESimulator.app
rm -rf "$APP"
mkdir -p "$APP/Contents/MacOS"
cp target/debug/RBLESimulator "$APP/Contents/MacOS/"
cp Info.plist "$APP/Contents/Info.plist"
codesign --force --deep --sign - "$APP" >/dev/null 2>&1
echo "已打包并签名: $APP"
