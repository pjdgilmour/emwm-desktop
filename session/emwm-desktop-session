#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-only
# A separate session: no edits to .profile, .Xprofile, .Xresources or XFCE.
EMWM_DESKTOP_PREFIX=/usr/lib/emwm-desktop
EMWM_DESKTOP_DATA=/usr/share/emwm-desktop
export EMWM_DESKTOP_PREFIX EMWM_DESKTOP_DATA
if [ -z "${DISPLAY:-}" ]; then
    echo 'EMWM Desktop requires an X11 display.' >&2
    exit 1
fi
# xinit without a display manager may not have a session bus yet.
if [ -z "${DBUS_SESSION_BUS_ADDRESS:-}" ]; then
    if [ -S "${XDG_RUNTIME_DIR:-/run/user/$(id -u)}/bus" ]; then
        DBUS_SESSION_BUS_ADDRESS="unix:path=${XDG_RUNTIME_DIR:-/run/user/$(id -u)}/bus"
        export DBUS_SESSION_BUS_ADDRESS
    else
        exec dbus-run-session -- "$0" "$@"
    fi
fi
PATH="$EMWM_DESKTOP_PREFIX/bin:$HOME/.local/bin:$PATH"
XDG_CURRENT_DESKTOP=EMWM
XDG_SESSION_DESKTOP=emwm-desktop
XDG_SESSION_TYPE=x11
XFILESEARCHPATH="$EMWM_DESKTOP_DATA/app-defaults/%N${XFILESEARCHPATH:+:$XFILESEARCHPATH}"
# Include this distribution's directory metadata while preserving the XDG path.
XDG_DATA_DIRS="$EMWM_DESKTOP_DATA:${XDG_DATA_DIRS:-/usr/local/share:/usr/share}"
export PATH XDG_CURRENT_DESKTOP XDG_SESSION_DESKTOP XDG_SESSION_TYPE XFILESEARCHPATH XDG_DATA_DIRS
state_dir=${XDG_STATE_HOME:-"$HOME/.local/state"}/emwm-desktop
mkdir -p "$state_dir"
[ ! -f "$state_dir/session.log" ] || mv -f "$state_dir/session.log" "$state_dir/session.log.old"
exec >>"$state_dir/session.log" 2>&1
# Existing user customizations remain authoritative. Do not require this file.
# shellcheck disable=SC1091
[ ! -f "$HOME/.Xprofile" ] || . "$HOME/.Xprofile"
config_dir=${XDG_CONFIG_HOME:-"$HOME/.config"}/emwm-desktop
EMWM_DESKTOP_COMPOSITOR=1
EMWM_DESKTOP_ICONS=1
EMWM_DESKTOP_TRAY=1
EMWM_DESKTOP_NETWORK=1
EMWM_DESKTOP_KEYRING=1
EMWM_DESKTOP_POLKIT=1
# shellcheck disable=SC1091
[ ! -f "$config_dir/session.conf" ] || . "$config_dir/session.conf"
# Appearance selections take precedence over a fixed GTK_THEME in old profiles.
if [ -f "$config_dir/appearance/session.sh" ]; then
    # shellcheck disable=SC1091
    . "$config_dir/appearance/session.sh"
    "$EMWM_DESKTOP_PREFIX/bin/emwm-appearance" --session
fi
# Restore the private binaries in PATH if a user profile replaced PATH entirely.
PATH="$EMWM_DESKTOP_PREFIX/bin:$PATH"
export PATH
xrdb -merge "$EMWM_DESKTOP_DATA/defaults.Xresources"
[ ! -f "$HOME/.Xdefaults" ] || xrdb -merge "$HOME/.Xdefaults"
[ ! -f "$HOME/.Xresources" ] || xrdb -merge "$HOME/.Xresources"
dbus-update-activation-environment --systemd DISPLAY XAUTHORITY XDG_CURRENT_DESKTOP XDG_SESSION_DESKTOP XDG_SESSION_TYPE PATH XFILESEARCHPATH XDG_DATA_DIRS || :
# Only these optional helpers are launched; no full desktop autostart scanner.
uid=$(id -u)
start_once() {
    pattern=$1
    shift
    if command -v "$1" >/dev/null 2>&1 && ! pgrep -u "$uid" -f "$pattern" >/dev/null 2>&1; then
        "$@" &
    fi
}
# Toolchest hosts the tray below its clock, including after a toolbox restart.
export EMWM_DESKTOP_TRAY
if [ "$EMWM_DESKTOP_COMPOSITOR" = 1 ]; then
    "$EMWM_DESKTOP_PREFIX/bin/emwm-compositor" &
    compositor_pid=$!
fi
if [ "$EMWM_DESKTOP_POLKIT" = 1 ]; then
    start_once '^(/[^ ]*/)?polkit-mate-authentication-agent-1([[:space:]]|$)' /usr/libexec/polkit-mate-authentication-agent-1
fi
if [ "$EMWM_DESKTOP_KEYRING" = 1 ]; then
    gnome-keyring-daemon --start --components=secrets,pkcs11 || :
fi
if [ "$EMWM_DESKTOP_NETWORK" = 1 ]; then
    start_once '^(/[^ ]*/)?nm-applet([[:space:]]|$)' nm-applet
fi
# Own only the processes started by this session. The compositor itself arbitrates
# its X screen selection; a compositor on another DISPLAY must not inhibit it.
cleanup_desktop() {
    [ -z "${icons_pid:-}" ] || kill "$icons_pid" 2>/dev/null || :
    [ -z "${compositor_pid:-}" ] || kill "$compositor_pid" 2>/dev/null || :
    [ -z "${sm_pid:-}" ] || kill "$sm_pid" 2>/dev/null || :
    [ -z "${icons_pid:-}" ] || wait "$icons_pid" 2>/dev/null || :
    [ -z "${compositor_pid:-}" ] || wait "$compositor_pid" 2>/dev/null || :
    [ -z "${sm_pid:-}" ] || wait "$sm_pid" 2>/dev/null || :
}
trap cleanup_desktop EXIT
trap 'exit 0' HUP INT TERM
"$EMWM_DESKTOP_PREFIX/bin/xmsm" &
sm_pid=$!
if [ "$EMWM_DESKTOP_ICONS" = 1 ]; then
    "$EMWM_DESKTOP_PREFIX/bin/emwm-desktop-icons" &
    icons_pid=$!
fi
wait "$sm_pid"
status=$?
sm_pid=
exit "$status"
