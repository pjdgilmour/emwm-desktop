#!/bin/sh
# Print the absolute path Cargo chose for the isolated X11 test client.
set -eu
crate=$(CDPATH='' cd -- "$(dirname -- "$0")/../.." && pwd)
cargo build --quiet --locked --manifest-path "$crate/Cargo.toml" --features x11-tests --example xmsm-probe --message-format=json | python3 -c '
import json, sys
print(next(item["executable"] for item in map(json.loads, sys.stdin) if item.get("executable") and item["target"]["name"] == "xmsm-probe"))
'
