#!/bin/sh
# Start the extracted package on a private X server, never on the real desktop.
set -eu
if [ "${1:-}" != --private-bus ]; then
    exec dbus-run-session -- sh "$0" --private-bus "$@"
fi
shift
package=$1
work=$(mktemp -d)
xpid=; session_pid=
cleanup() {
    [ -z "$session_pid" ] || /bin/kill -KILL -- "-$session_pid" 2>/dev/null || :
    [ -z "$xpid" ] || kill "$xpid" 2>/dev/null || :
    if [ "$1" != 0 ]; then cat "$work/home/.local/state/emwm-desktop/session.log" "$work/launcher.log" 2>/dev/null || :; fi
    rm -rf "$work"
}
trap 'cleanup $?' EXIT
trap 'exit 1' INT TERM HUP
dpkg-deb -x "$package" "$work/root"
mkdir -p "$work/home/.config/emwm-desktop"
python3 - "$work" <<'PY'
from pathlib import Path
import sys
work=Path(sys.argv[1]); root=work/'root'
prefix=root/'usr/lib/emwm-desktop'; data=root/'usr/share/emwm-desktop'
# Relocate only this temporary test copy; package paths stay fixed at install time.
for path in [root/'usr/bin/emwm-desktop-session',data/'app-defaults/XmSm',data/'app-defaults/XmToolbox']:
    text=path.read_text().replace('/usr/lib/emwm-desktop',str(prefix)).replace('/usr/share/emwm-desktop',str(data))
    path.write_text(text)
with (data/'defaults.Xresources').open('a') as stream:
    stream.write(f'\nEmwm*configFile: {data}/system.emwmrc\n')
    stream.write(f'XmToolbox.rcFile: {data}/toolboxrc\n')
    stream.write(f'XmToolbox.applicationsMenuFile: {data}/emwm-applications.menu\n')
(work/'home/.config/emwm-desktop/session.conf').write_text('EMWM_DESKTOP_POLKIT=0\nEMWM_DESKTOP_KEYRING=0\nEMWM_DESKTOP_NETWORK=0\n')
PY
Xvfb -displayfd 3 -screen 0 1024x768x24 -nolisten tcp -noreset 3>"$work/display" >"$work/x.log" 2>&1 &
xpid=$!
i=0
while [ ! -s "$work/display" ]; do i=$((i+1)); [ "$i" -lt 50 ] || exit 1; sleep .1; done
DISPLAY=":$(cat "$work/display")"
export DISPLAY HOME="$work/home" XENVIRONMENT="$work/empty" XDG_CONFIG_HOME="$work/home/.config" XDG_STATE_HOME="$work/home/.local/state"
unset SESSION_MANAGER
: >"$XENVIRONMENT"
setsid sh "$work/root/usr/bin/emwm-desktop-session" >"$work/launcher.log" 2>&1 &
session_pid=$!
mapped_class() {
    # X window IDs are single whitespace-free hexadecimal words.
    # shellcheck disable=SC2013
    for wid in $(awk -v class="\"$1\"" 'index($0, class) { print $1 }' "$work/windows"); do
        if xwininfo -id "$wid" | grep 'Map State: IsViewable' >/dev/null; then return 0; fi
    done
    return 1
}
i=0
while :; do
    xwininfo -root -tree >"$work/windows"
    if mapped_class XmTray && mapped_class XmToolbox && xprop -root _NET_SUPPORTING_WM_CHECK | grep '0x' >/dev/null; then break; fi
    i=$((i+1)); [ "$i" -lt 100 ] || exit 1
    kill -0 "$session_pid"
    sleep .1
done
xprop -root _NET_SUPPORTING_WM_CHECK | grep '0x'
kill -0 "$session_pid"
if [ -n "${EMWM_DESKTOP_CAPTURE:-}" ]; then import -display "$DISPLAY" -window root "$EMWM_DESKTOP_CAPTURE"; fi
printf '%s\n' 'PASS: extracted package starts xmsm, EMWM, Toolchest and xmtray on an isolated X server'
