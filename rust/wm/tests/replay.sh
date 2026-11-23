#!/bin/sh
set -eu
project=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
EMWM_PARITY_TREE=$(realpath "$1")
original=$(realpath "$2")
native=$(realpath "$3")
export EMWM_PARITY_TREE
cat > "$work/inputs.jsonl" <<'CASES'
{"name":"no-display","args":["no-display"]}
{"name":"decoration","args":["decoration"]}
{"name":"tiling","args":["tiling"]}
{"name":"defaults","args":["defaults"]}
CASES
tool=${RUST_ANYTHING:-rust-anything}
EMWM_BINARY="$original" "$tool" app record "$work" --name emwm --timeout 90 --no-traps --inputs "$work/inputs.jsonl" -- "$project/run-case.sh"
EMWM_BINARY="$native" "$tool" app replay "$work" --build "$project/run-case.sh"
