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
tray_lib="$utils/src/rust-target/release/libemwm_tray.a"
config_lib="$utils/src/rust-target/release/libemwm_toolbox_config.a"
make -C "$utils/src" -f ../mf/Makefile.Linux TRAY_CRATE="$project/rust/tray" \
    TOOLBOX_CONFIG_CRATE="$project/rust/emwm-utils" "$tray_lib" "$config_lib"
# Compiler and pkg-config flags intentionally expand into separate arguments.
# shellcheck disable=SC2046,SC2086
${CC:-cc} ${CFLAGS:--O2 -Wall} -I"$utils/src" \
    -DRCDIR='"/etc/X11"' \
    $(pkg-config --cflags garcon-1 gio-unix-2.0) \
    "$project/tests/toolchest-tray.c" \
    "$utils/src/common.c" "$utils/src/wswitch.c" \
    "$utils/src/tbxdg.c" "$utils/src/tbxdgmenu.c" \
    -o "$work/test-tray" "$config_lib" "$tray_lib" -ldl -lpthread -lm -lXm -lXt -lXrender -lXrandr -lX11 -lXtst \
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
