#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-only
set -eu
wm=$(realpath "$1")
compositor=$(realpath "$2")
rc=$(realpath "$3")
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cc -Wall -Wextra "$(dirname "$0")/overview.c" -o "$work/probe" -lX11 -lXtst
for backend in xrender glx none; do
    HOME="$work" xvfb-run -a -s '-screen 0 1024x768x24 -noreset -nolisten tcp' sh -eu -c '
        unset SESSION_MANAGER XENVIRONMENT
        export XDG_CONFIG_HOME="$HOME/.config"
        "$1" -xrm "Emwm*configFile: $3" -xrm "Emwm*workspaceCount: 4" \
            -xrm "Emwm*interactivePlacement: False" -xrm "Emwm*clientAutoPlace: False" >"$HOME/wm.log" 2>&1 &
        wm=$!
        if [ "$4" = none ]; then
            trap "kill $wm 2>/dev/null || true" EXIT
            "$HOME/probe" scroll-only
            exit
        fi
        "$2" --backend "$4" >"$HOME/compositor.log" 2>&1 &
        cmp=$!
        trap "kill $cmp $wm 2>/dev/null || true" EXIT
        "$HOME/probe"
        grep "backend=$4" "$HOME/compositor.log" >/dev/null
        if [ "$4" = glx ]; then ! grep -q "backend=xrender" "$HOME/compositor.log"; fi
    ' sh "$wm" "$compositor" "$rc" "$backend" || { cat "$work/wm.log" "$work/compositor.log" >&2; exit 1; }
done
