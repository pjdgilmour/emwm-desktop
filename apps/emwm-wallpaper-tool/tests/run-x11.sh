#!/bin/sh
# Only our private X server and WM processes are touched.
set -eu
probe=$1
tool=$2
: "${EMWM_BINARY:?Set EMWM_BINARY to the EMWM executable}"
work=$(mktemp -d)
xpid=; wmpid=
cleanup() {
    [ -z "$wmpid" ] || kill -KILL "$wmpid" 2>/dev/null || :
    [ -z "$xpid" ] || kill "$xpid" 2>/dev/null || :
    if [ "$1" != 0 ]; then cat "$work"/*.log 2>/dev/null || :; fi
    rm -rf "$work"
}
trap 'cleanup $?' EXIT
trap 'exit 1' HUP INT TERM
Xvfb -displayfd 3 -screen 0 800x600x24 -nolisten tcp -noreset 3>"$work/display" >"$work/xvfb.log" 2>&1 &
xpid=$!
i=0
while [ ! -s "$work/display" ]; do
    i=$((i+1)); [ "$i" -lt 50 ] || exit 1
    sleep .1
done
export DISPLAY=:$(cat "$work/display")
export HOME="$work" XENVIRONMENT="$work/empty" LC_ALL=C.UTF-8
: >"$XENVIRONMENT"
unset SESSION_MANAGER
"$probe" fixture "$HOME"
"$tool" --migrate
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
if grep -i 'Unable to get image' "$work"/wm-*.log; then exit 1; fi
echo 'PASS: immediate IPC, WM restart and fresh login; spaces, quotes, backslash and UTF-8 paths'
