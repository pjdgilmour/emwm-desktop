#!/bin/sh
# Only our private X server and WM processes are touched.
set -eu
probe=$1
tool=$2
render_probe=$3
ui_probe=$4
cargo=$5
manifest=$6
cargo_target=$7
: "${EMWM_BINARY:?Set EMWM_BINARY to the EMWM executable}"
work=$(mktemp -d)
xpid=; wmpid=
cleanup() {
    [ -z "$wmpid" ] || kill -KILL "$wmpid" 2>/dev/null || :
    [ -z "$xpid" ] || kill "$xpid" 2>/dev/null || :
    if [ "$1" != 0 ]; then tail -n 80 "$work"/*.log 2>/dev/null || :; fi
    rm -rf "$work"
}
trap 'cleanup $?' EXIT
trap 'exit 1' HUP INT TERM
# Build before entering the private HOME. Cargo emits the test executable's
# real path, which includes a target- and toolchain-dependent hash.
"$cargo" test --locked --release --no-run --message-format=json --manifest-path "$manifest" \
    --target-dir "$cargo_target" > "$work/artifacts.jsonl"
rust_ui=$(/usr/bin/python3 -c 'import json, sys
for line in sys.stdin:
    artifact = json.loads(line)
    if artifact.get("profile", {}).get("test") and artifact.get("executable"):
        print(artifact["executable"])
        break' < "$work/artifacts.jsonl")
test -x "$rust_ui"
Xvfb -displayfd 3 -screen 0 800x600x24 -nolisten tcp -noreset 3>"$work/display" >"$work/xvfb.log" 2>&1 &
xpid=$!
i=0
while [ ! -s "$work/display" ]; do
    i=$((i+1)); [ "$i" -lt 50 ] || exit 1
    kill -0 "$xpid"
    sleep .1
done
export DISPLAY=:$(cat "$work/display")
export HOME="$work" XENVIRONMENT="$work/empty" LC_ALL=C.UTF-8
: >"$XENVIRONMENT"
unset SESSION_MANAGER
"$render_probe" layout
"$probe" fixture "$HOME"
"$tool" --migrate
timeout 20 dbus-run-session -- "$ui_probe"
timeout 20 dbus-run-session -- "$rust_ui" ui::tests::monitor_dialog --ignored --exact
# A new login reconstructs RESOURCE_MANAGER from .Xresources, including cpp.
xrdb -remove
xrdb -merge "$HOME/.Xresources"
xrdb -query | grep 'XTerm\*faceSize:.*13'
wait_image() {
    i=0
    until "$probe" pixel >"$work/pixel.log" 2>&1; do
        i=$((i+1)); [ "$i" -lt 50 ] || return 1
        kill -0 "$wmpid"
        sleep .1
    done
    cat "$work/pixel.log"
}
wait_render() {
    i=0
    until "$render_probe" pixel "$1" >"$work/render-pixel.log" 2>&1; do
        i=$((i+1)); [ "$i" -lt 50 ] || return 1
        kill -0 "$wmpid"
        sleep .1
    done
}
for pass in 1 2 3; do
    # Pass 2 restarts only the WM; pass 3 simulates another login.
    if [ "$pass" = 3 ]; then
        xrdb -remove
        xrdb -merge "$HOME/.Xresources"
    fi
    "$EMWM_BINARY" >"$work/wm-$pass.log" 2>&1 &
    wmpid=$!
    wait_image
    if [ "$pass" = 1 ]; then
        tellmwm backdrop ws0 -b '#102030' none
        sleep .2
        if "$probe" pixel >/dev/null 2>&1; then echo 'backdrop did not clear'; exit 1; fi
        argument=$("$probe" argument "$(cat "$HOME/image-path")")
        tellmwm backdrop ws0 "$argument"
        wait_image
    fi
    kill -KILL "$wmpid"
    wait "$wmpid" 2>/dev/null || :
    wmpid=
    i=0
    while "$probe" pixel >/dev/null 2>&1; do
        i=$((i+1)); [ "$i" -lt 50 ] || exit 1
        sleep .1
    done
done
for mode in fit zoom stretch center tile monitors; do
    "$EMWM_BINARY" >"$work/wm-$mode.log" 2>&1 &
    wmpid=$!
    wait_image
    argument=$("$render_probe" fixture "$HOME" "$mode")
    if [ "$mode" != monitors ]; then
        # The Rust renderer must reproduce the content-addressed RGBA PNG from C++.
        cp "$HOME/.config/emwm/backdrops.resources" "$work/expected.resources"
        "$tool" --refresh-layout
        cmp "$work/expected.resources" "$HOME/.config/emwm/backdrops.resources"
    fi
    xrdb -merge -nocpp "$HOME/.config/emwm/backdrops.resources"
    tellmwm backdrop ws0 "$argument"
    wait_render "$mode"
    kill -KILL "$wmpid"
    wait "$wmpid" 2>/dev/null || :
    wmpid=
    # Reconstruct resources as at login and check the same composed pixels.
    xrdb -remove
    xrdb -merge "$HOME/.Xresources"
    "$EMWM_BINARY" >>"$work/wm-$mode.log" 2>&1 &
    wmpid=$!
    wait_render "$mode"
    kill -KILL "$wmpid"
    wait "$wmpid" 2>/dev/null || :
    wmpid=
    # Restore the legacy fixture for the next immediate-application check.
    rm "$HOME/.config/emwm/backdrops.resources"
    "$tool" --migrate
done
if grep -i 'Unable to get image' "$work"/wm-*.log; then exit 1; fi
echo 'PASS: immediate IPC, WM restart and fresh login; spaces, quotes, backslash and UTF-8 paths'
echo 'PASS: fit, zoom, stretch, center, tile and multi-monitor pixels after IPC and fresh login'
