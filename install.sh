#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-only
set -eu
project_dir=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
exec python3 "$project_dir/scripts/install.py" "$@"
