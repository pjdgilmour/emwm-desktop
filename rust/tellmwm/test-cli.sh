#!/bin/sh
# Compare the original and the port under a new Xvfb. Run from any directory.
set -eu
repo=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
ref=$(mktemp -d)
trap 'rm -rf "$ref"' EXIT
cc -O2 "$repo"/../../third_party/tellmwm/tellmwm.c "$repo"/../../third_party/tellmwm/wsmutil.c -lX11 -o "$ref/tellmwm"
native=${1:?path to Rust tellmwm}
case "$native" in /*) ;; *) native=$PWD/$native ;; esac
export EMWM_TELLMWM_REFERENCE=$ref/tellmwm
xvfb-run -a sh -ec 'xdpyinfo >/dev/null; "$1" app replay "$2" --build "$3"; "$1" app replay "$2" --build "$4"' sh "${RUST_ANYTHING:-rust-anything}" "$repo" "$ref/tellmwm" "$native"
