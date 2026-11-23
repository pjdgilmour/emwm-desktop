#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-only
# Install the appearance utility independently of the complete desktop package.
set -eu
if [ "$(id -u)" -ne 0 ]; then
    echo 'Execute: sudo ./apps/emwm-appearance/install-system.sh' >&2
    exit 1
fi
source_dir=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
/usr/bin/apt-get install cargo rustc build-essential pkg-config libgtk-3-dev xsettingsd pkexec x11-xserver-utils
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cargo build --locked --release --manifest-path "$source_dir/../../rust/appearance/app/Cargo.toml" --target-dir "$work/target"
dest=/usr/lib/emwm-desktop/appearance
install -o root -g root -d -m 755 "$dest"
install -o root -g root -m 755 "$work/target/release/emwm-appearance" "$work/target/release/emwm-appearance-login" "$dest/"
install -o root -g root -m 755 "$source_dir/emwm-appearance" /usr/bin/emwm-appearance
install -o root -g root -m 644 "$source_dir/emwm-appearance.desktop" /usr/share/applications/emwm-appearance.desktop
printf '%s\n' 'Instalado. Abra Aparência do EMWM no menu de aplicativos ou execute emwm-appearance.'
