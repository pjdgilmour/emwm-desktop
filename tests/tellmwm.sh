#!/bin/sh
# Replay workspace mutations against equivalent WMs on separate private displays.
set -eu
wm=$(realpath "$1")
native=$(realpath "$2")
src=$(CDPATH='' cd -- "$(dirname -- "$0")/../third_party/tellmwm" && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cc "$src/tellmwm.c" "$src/wsmutil.c" -lX11 -o "$work/reference"
for client in "$work/reference" "$native"; do
    result="$work/$(basename "$client").log"
    testhome=$(mktemp -d "$work/profile.XXXXXX")
    HOME="$testhome" xvfb-run -a -s "-screen 0 1024x768x24 -noreset -nolisten tcp" sh -eu -c '
        unset SESSION_MANAGER
        "$1" -xrm "Emwm*workspaceCount: 2" >"$HOME/wm.log" 2>&1 &
        pid=$!
        trap "kill -KILL $pid 2>/dev/null || :; wait $pid 2>/dev/null || :" EXIT
        cli=$2
        until "$cli" >"$HOME/info" && grep -q "^Current Workspace: " "$HOME/info"; do
            i=$((${i:-0}+1)); [ "$i" -lt 80 ] || exit 1; sleep .05
        done
        cat "$HOME/info"
        wait_for () {
            i=0
            until "$cli" | grep -F -- "$1" >/dev/null; do
                i=$((i+1)); [ "$i" -lt 80 ] || exit 1; sleep .05
            done
            "$cli"
        }
        fail () { if "$cli" "$@"; then exit 1; fi; }
        fail setws
        fail setws NO_SUCH_WORKSPACE
        fail namews ws0
        fail backdrop ws0
        fail backdrop ws0 -f
        fail backdrop ws0 a b
        "$cli" setws ws1
        wait_for "Current Workspace: ws1"
        "$cli" namews ws1 "Mesa azul"
        wait_for "Mesa azul"
        "$cli" backdrop ws1 -f red -b "#112233" none
        wait_for "Background: #112233"
        "$cli" addws "Extra Mesa"
        wait_for "Extra Mesa"
        ws=$("$cli" | sed -n "s/^Workspace \"\(.*\)\"/\1/p" | tail -1)
        "$cli" delws "$ws"
        i=0
        while "$cli" | grep -F "Extra Mesa" >/dev/null; do
            i=$((i+1)); [ "$i" -lt 80 ] || exit 1; sleep .05
        done
        "$cli"
    ' sh "$wm" "$client" >"$result" 2>&1 || { cat "$result" "$testhome/wm.log" >&2; exit 1; }
done
diff -u "$work/reference.log" "$work/$(basename "$native").log"
echo "PASS: workspace reports, mutations, colors and errors match tellmwm 1.2"
