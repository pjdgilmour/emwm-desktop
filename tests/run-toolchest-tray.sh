#!/bin/sh
# Build against a patched emwm-utils snapshot; use a private X server.
set -eu
project=$(CDPATH='' cd -- "$(dirname "$0")/.." && pwd)
utils=$(CDPATH='' cd -- "$1" && pwd)
work=$(mktemp -d)
xpid=
cleanup() {
    [ -z "$xpid" ] || kill "$xpid" 2>/dev/null || :
    rm -rf "$work"
}
trap cleanup EXIT
trap 'exit 1' HUP INT TERM
crate="$project/rust/toolbox"
target="$utils/src/rust-test-target"
EMWM_RCDIR="$utils/src" cargo build --release --locked --features test-probes --lib --manifest-path "$crate/Cargo.toml" --target-dir "$target"
# shellcheck disable=SC2086,SC2046
${CC:-cc} ${CFLAGS:--O2 -Wall} -I"$crate/tests" "$project/tests/toolchest-tray.c" \
    "$target/release/libemwm_toolbox.a" -o "$work/test-tray" \
    -lXm -lXt -lXrender -lXrandr -lX11 -lXtst -ldl -lpthread -lm \
    $(pkg-config --libs garcon-1 gio-unix-2.0)
Xvfb -displayfd 3 -screen 0 1024x768x24 -nolisten tcp -noreset \
    3>"$work/display" >"$work/x.log" 2>&1 &
xpid=$!
i=0
while [ ! -s "$work/display" ]; do
    i=$((i+1)); [ "$i" -lt 50 ] || { cat "$work/x.log"; exit 1; }
    sleep .1
done
export DISPLAY=":$(cat "$work/display")" HOME="$work" XENVIRONMENT="$work/empty"
: >"$XENVIRONMENT"
"$work/test-tray"
"$work/test-tray" -horizontal
"$work/test-tray" -xrm '*dateTimeDisplay: False' -xrm '*workspaceSwitcher: False'
"$work/test-tray" -xrm '*systemTray: False'
