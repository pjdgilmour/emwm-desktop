#!/bin/sh
set -eu
app=$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)
work=$(mktemp -d)
xpid=; wmpid=
cleanup() {
    [ -z "$wmpid" ] || kill "$wmpid" 2>/dev/null || :
    [ -z "$xpid" ] || kill "$xpid" 2>/dev/null || :
    if [ "$1" != 0 ]; then cat "$work"/*.log "$work/home/.local/state/emwm-desktop/appearance-xsettingsd.log" 2>/dev/null || :; fi
    rm -rf "$work"
}
trap 'cleanup $?' EXIT
trap 'exit 1' HUP INT TERM
Xvfb -displayfd 3 -screen 0 900x800x24 -nolisten tcp -noreset 3>"$work/display" >"$work/x.log" 2>&1 &
xpid=$!
i=0
while [ ! -s "$work/display" ]; do i=$((i+1)); [ "$i" -lt 50 ] || exit 1; sleep .1; done
DISPLAY=:$(cat "$work/display")
HOME="$work/home"
mkdir -p "$HOME"
export DISPLAY HOME GSETTINGS_BACKEND=memory
unset XDG_CONFIG_HOME XDG_DATA_HOME XDG_STATE_HOME GTK_THEME SESSION_MANAGER
printf '%s\n' '! keep backdrops.resources reference' > "$HOME/.Xresources"
if [ -n "${EMWM_BINARY:-}" ]; then
    "$EMWM_BINARY" -display "$DISPLAY" -xrm 'Emwm*interactivePlacement: False' -xrm 'Emwm*clientAutoPlace: False' >"$work/wm.log" 2>&1 &
    wmpid=$!
    sleep .5
fi
timeout 20 dbus-run-session -- /usr/bin/python3 "$app/tests/gui-smoke.py"
timeout 20 dbus-run-session -- /usr/bin/python3 "$app/tests/wallpaper-smoke.py"
