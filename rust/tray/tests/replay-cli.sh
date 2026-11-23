#!/bin/sh
# Compare both binaries on a private display using the same Xt resource database.
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
{"name":"help","args":["--help"]}
{"name":"unknown","args":["--unknown"]}
{"name":"icons-too-small","args":["-iconsize","7"]}
{"name":"icons-too-large","args":["-iconsize","129"]}
{"name":"spacing-negative","args":["-spacing","-1"]}
{"name":"spacing-too-large","args":["-spacing","65"]}
{"name":"minimum-width","args":["-xrm","*minimumWidth: 31"]}
{"name":"unavailable-display","args":["-display",":98765"]}
CASES
tool=${RUST_ANYTHING:-rust-anything}
"$tool" app record "$work" --no-traps --inputs "$work/inputs" -- "$original"
"$tool" app replay "$work" --build "$ported"
