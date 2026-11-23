#!/bin/sh
# Stage the appearance extensions next to the GTK modules in a package or test tree.
set -eu
root=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
target=${1:?cargo target directory required}
destination=${2:?appearance module directory required}
for unit in core desktop compositor wallpaper; do
    module=appearance_${unit}_rs
    cargo build --release --locked --manifest-path "$root/$unit/Cargo.toml" --target-dir "$target"
    install -d -m 755 "$destination/$module"
    install -m 644 "$root/$unit/python/$module/__init__.py" "$destination/$module/"
    install -m 644 "$target/release/lib$module.so" "$destination/$module/_native.abi3.so"
done
