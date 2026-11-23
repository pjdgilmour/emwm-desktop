#!/bin/sh
# Compare the pinned parser and its Rust replacement through the Toolchest C API.
set -eu
original=$(realpath "$1")/src
crate=$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cargo build --locked --release --manifest-path "$crate/Cargo.toml" --target-dir "$work/target"
cc -Wall -Wextra -I"$original" "$crate/tests/parser.c" "$original/tbparse.c" -o "$work/original"
cc -Wall -Wextra -I"$original" "$crate/tests/parser.c" "$work/target/release/libemwm_toolbox_config.a" -ldl -lpthread -lm -o "$work/ported"
mkdir "$work/record"
rust-anything app record "$work/record" --no-traps --name toolbox-config --inputs "$crate/tests/cases.jsonl" -- "$work/original"
rust-anything app replay "$work/record" --build "$work/ported"
for config in "$original/toolboxrc" "$crate/../../config/toolboxrc"; do
    "$work/original" "$config" > "$work/expected"
    "$work/ported" "$config" | cmp "$work/expected" -
done
