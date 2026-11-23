#!/bin/sh
# Run under xvfb-run -a to compare against a clean Xt resource database.
set -eu
original=$(realpath "$1")
ported=$(realpath "$2")
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
mkdir "$work/home"
export HOME="$work/home" XENVIRONMENT="$work/empty"
unset SESSION_MANAGER
: >"$XENVIRONMENT"
cat >"$work/inputs" <<'CASES'
{"name":"version","args":["-version"]}
{"name":"horizontal","args":["-horizontal","-version"]}
{"name":"vertical","args":["+horizontal","-version"]}
{"name":"manual-rc","args":["-version","-rcfile","toolboxrc"]}
{"name":"applications-off","args":["-noapplications","-version"]}
{"name":"applications-on","args":["-applications","-version"]}
{"name":"tray-resource","args":["-xrm","*systemTray: False","-version"]}
{"name":"unavailable-display","args":["-display",":98765","-version"]}
CASES
tool=${RUST_ANYTHING:-rust-anything}
"$tool" app record "$work" --no-traps --inputs "$work/inputs" -- "$original"
"$tool" app replay "$work" --build "$ported"
