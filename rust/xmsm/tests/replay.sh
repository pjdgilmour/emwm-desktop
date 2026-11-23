#!/bin/sh
# Records the non-setuid reference in Xvfb and replays each case against the port.
set -eu
crate=$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
original=$(realpath "${1:?unprivileged reference xmsm}")
ported=$(realpath "${2:?unprivileged Rust xmsm}")
probe=$(sh "$crate/tests/support/probe.sh")
export XMSM_TEST_PROBE=$probe XMSM_TEST_DRIVER=$crate/tests/session.py
cat >"$work/case" <<'WRAP'
#!/bin/sh
exec xvfb-run -a -s '-screen 0 1024x768x24 -noreset -nolisten tcp' python3 "$XMSM_TEST_DRIVER" "$@"
WRAP
chmod +x "$work/case"
mkdir "$work/record"
for case in version missing-display failed-launch signal message no-shade duplicates lock-denied disabled-suspend stale; do
    printf '{"name":"%s","args":["%s"]}\n' "$case" "$case"
done >"$work/inputs"
tool=${RUST_ANYTHING:-rust-anything}
export XMSM_TEST_BINARY=$original
"$tool" app record "$work/record" --no-traps --timeout 15 --inputs "$work/inputs" -- "$work/case"
export XMSM_TEST_BINARY=$ported
"$tool" app replay "$work/record" --build "$work/case" || { cat "$work/record/rust/app-cases/replay-report.md"; exit 1; }
