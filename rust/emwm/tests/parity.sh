#!/bin/sh
# Usage: parity.sh ORIGINAL_EMWM_SOURCE PATCHED_EMWM_SOURCE
# Build the two C-ABI probes against the same X server replies.
set -eu
original=$(realpath "$1")
patched=$(realpath "$2")
crate=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cargo build --release --locked --manifest-path "$crate/Cargo.toml" --target-dir "$work/target"
cc -O2 -I"$original/src" "$crate/tests/query.c" "$original/src/WmXinerama.c" -o "$work/original"
cc -O2 -I"$patched/src" "$crate/tests/query.c" "$patched/src/WmXinerama.c" \
    "$work/target/release/libemwm_monitor.a" -lXrender -lX11 -ldl -lpthread -lm -o "$work/ported"
for scenario in layout absent disabled failed-query change; do
    printf '{"name":"%s","args":["%s"]}\n' "$scenario" "$scenario" >>"$work/inputs.jsonl"
done
"${RUST_ANYTHING:-rust-anything}" app record "$work" --no-traps --inputs "$work/inputs.jsonl" -- "$work/original"
"${RUST_ANYTHING:-rust-anything}" app replay "$work" --build "$work/ported"
