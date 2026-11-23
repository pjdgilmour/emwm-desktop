#!/bin/sh
# Extract the original monitor comparison from pinned EMWM and gate the
# shipping Rust calculation against it through a by-value C test adapter.
set -eu
source=$(realpath "$1")/src/WmTile.c
crate=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
tool=${RUST_ANYTHING:-rust-anything}
export CARGO_TARGET_DIR="$work/target" XDG_CACHE_HOME="$work/cache"
python3 - "$source" "$work" <<'PY'
import itertools, json, pathlib, sys
s = pathlib.Path(sys.argv[1]).read_text()
d = pathlib.Path(sys.argv[2])
c = s[s.index("#define MIN"):s.index("/* Zero")] + s[s.index("typedef struct"):s.index("static Atom")]
c += s[s.index("static long long Intersection"):s.index("static TileRect ClientMonitor")]
c += """
long long tile_area(TileRect client, TileRect candidate) {
    return Intersection(client, candidate);
}
long long tile_distance(TileRect client, TileRect candidate) {
    TileRect best = {0}; long long area = -1, distance = -1;
    ChooseMonitor(client, candidate, &best, &area, &distance);
    return distance;
}
int tile_selected(TileRect client, TileRect candidate, long long area, long long distance) {
    TileRect best = {0};
    ChooseMonitor(client, candidate, &best, &area, &distance);
    return best.width > 0;
}
"""
(d / "geometry.c").write_text(c)
def rect(values):
    return dict(zip(("x", "y", "width", "height"), values))
client = rect((-50, 10, 101, 61))
with (d / "inputs").open("w") as f:
    for bounds in [(-10, 10, 40, 29), (50, 70, 10, 5), (51, 70, 40, 60),
                   (51, 70, 0, 50), (-50000, -12000, 100020, 45009)]:
        candidate = rect(bounds)
        for name in ("tile_area", "tile_distance"):
            print(json.dumps({"fn":name, "args":[client, candidate]}), file=f)
        for area, distance in itertools.product([-1, 0, 1, 1160, 6161], [0, 1, 2**33]):
            print(json.dumps({"fn":"tile_selected", "args":[client, candidate, area, distance]}), file=f)
PY
"$tool" bridge "$work" geometry.c
bridge=$work/rust/geometry-rs
"$tool" record "$bridge" --inputs "$work/inputs"
cp "$crate/src/tiling.rs" "$bridge/src/tiling.rs"
cat > "$bridge/src/port.rs" <<'RS'
#![forbid(unsafe_code)]
use crate::TileRect;
#[path = "tiling.rs"]
mod tiling;
pub fn tile_area(client: TileRect, candidate: TileRect) -> i64 {
    tiling::tile_score(client, candidate).0
}
pub fn tile_distance(client: TileRect, candidate: TileRect) -> i64 {
    tiling::select_tile_monitor(client, candidate, -1, -1).map_or(-1, |(_, distance)| distance)
}
pub fn tile_selected(client: TileRect, candidate: TileRect, area: i64, distance: i64) -> i32 {
    tiling::select_tile_monitor(client, candidate, area, distance).is_some().into()
}
RS
cargo fmt --manifest-path "$bridge/Cargo.toml"
"$tool" build "$bridge"
"$tool" diff "$bridge"
"$tool" gate "$bridge"
