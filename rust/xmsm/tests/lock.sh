#!/bin/sh
# This runs the Rust unit-test executable with a synthetic authentication peer.
# The production executable has no fixture account or authentication switch.
set -eu
crate=$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cargo test --locked --no-run --manifest-path "$crate/Cargo.toml" --message-format=json > "$work/build.jsonl"
binary=$(python3 - "$work/build.jsonl" <<'PY'
import json,sys
for line in open(sys.argv[1]):
    item=json.loads(line)
    if item.get('target',{}).get('name')=='xmsm' and item.get('executable'):
        print(item['executable'])
PY
)
probe=$(sh "$crate/tests/support/probe.sh")
mkdir "$work/home"
cat > "$work/resources" <<RES
*windowManager: /bin/true
*launcher: /bin/true
*showReboot: False
*showShutdown: False
*enableSuspend: False
*numLockState: keep
*unlockScreenTimeout: 10
RES
HOME="$work/home" XENVIRONMENT="$work/resources" XMSM_TEST_SYNTHETIC=1 \
    xvfb-run -a -s '-screen 0 1024x768x24 -noreset -nolisten tcp' sh -ec '
        bin=$1
        probe=$2
        "$bin" --ignored --exact tests::isolated_lock_session --nocapture >"$HOME/log" 2>&1 &
        pid=$!
        trap '\''kill "$pid" 2>/dev/null || :; wait "$pid" 2>/dev/null || : '\'' EXIT
        wait_for(){ n=0; until "$@" >/dev/null; do n=$((n+1)); if [ "$n" -gt 100 ]; then cat "$HOME/log" >&2; return 1; fi; sleep .03; done; }
        unlocked(){ ! "$probe" grab; }
        wait_for "$probe" LOCK
        wait_for "$probe" grab
        "$probe" key x
        "$probe" key Return
        sleep .15
        "$probe" grab
        "$probe" LOGOUT
        sleep .05
        if "$probe" dialog; then exit 1; fi
        for k in d e m o Return; do "$probe" key "$k"; done
        wait_for unlocked
        "$probe" LOGOUT
        wait_for "$probe" dialog
        "$probe" ok
        wait "$pid"
    ' sh "$binary" "$probe"
echo 'PASS: lock grabs input, rejects wrong password, unlocks with the synthetic account and logs out'
