#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-only
set -eu
project_dir=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
# Bootstrap the build tool on a fresh Debian installation.
if ! command -v cargo >/dev/null || ! command -v rustc >/dev/null; then
    yes=
    for arg do
        case $arg in
            -h|--help) cat "$project_dir/rust/desktop/src/install-help.txt"; exit 0 ;;
            --build-only) echo 'O build requer cargo e rustc.' >&2; exit 1 ;;
            --yes) yes=-y ;;
        esac
    done
    if [ "$(id -u)" = 0 ]; then echo 'Execute como usuário normal.' >&2; exit 1; fi
    sudo apt-get update
    sudo apt-get install ${yes:+"$yes"} cargo rustc build-essential
fi
export EMWM_BUILD_ROOT="$project_dir"
exec cargo run --quiet --locked --manifest-path "$project_dir/rust/desktop/Cargo.toml" --bin emwm-install -- "$@"
