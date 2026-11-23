#!/bin/sh
# Build the private CPython extension and stage its importable package.
set -eu
crate=$(CDPATH='' cd -- "$(dirname -- "$0")/../rust/gradient-rs" && pwd)
target=${1:?cargo target directory required}
destination=${2:?appearance module directory required}/gradient_rs
cargo build --manifest-path "$crate/Cargo.toml" --release --locked --target-dir "$target"
install -d -m 755 "$destination"
install -m 644 "$crate/python/gradient_rs/__init__.py" "$destination/"
install -m 644 "$target/release/libgradient_rs.so" "$destination/_native.abi3.so"
