#!/bin/sh
# Replay the C++ CLI contract on the Rust binary, without connecting to a desktop.
set -eu
app=$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)
binary=$(realpath "${1:?supply the Rust emwm-compositor binary}")
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
c++ -std=c++17 -O2 "$app/src/main.cpp" "$app/src/renderer.cpp" -o "$work/original" \
    $(pkg-config --cflags --libs x11 xcomposite xdamage xfixes xrender xext xrandr gl)
export DISPLAY=
rust-anything app record "$work" --no-traps --inputs "$app/../../rust/compositor/cli-cases.jsonl" -- "$work/original"
rust-anything app replay "$work" --build "$binary"
