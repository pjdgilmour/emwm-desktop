#!/bin/sh
# Gate the byte grammar against the pinned C parser, including linked entry contents.
set -eu
original=$(realpath "$1")/src
crate=$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
tool=${RUST_ANYTHING:-rust-anything}
export XDG_CACHE_HOME="$work/cache"
cp "$original/tbparse.h" "$work/tbparse.h"
# Include the original grammar and its list construction; replay.sh exercises file I/O.
sed '/^ \* Parses a toolbox menu file/,$d' "$original/tbparse.c" | sed '$d' > "$work/menu.c"
cat >> "$work/menu.c" <<'C'
/* The list of linked structs is opaque to rust-anything. Summarize its
 * ordered fields as bytes so a scalar recording can capture the whole tree. */
static unsigned hash_byte(unsigned hash, unsigned char byte)
{
    return (hash ^ byte) * 16777619u;
}
static unsigned hash_field(unsigned hash, const char *text)
{
    if (!text) return hash_byte(hash, 255);
    while (*text) hash = hash_byte(hash, (unsigned char)*text++);
    return hash_byte(hash, 0);
}
unsigned menu_digest(const char *text)
{
    if (!text) return 0;
    unsigned hash = 2166136261u;
    size_t len = strlen(text);
    buffer = calloc(len + 3, 1);
    if (!buffer) return 0;
    memcpy(buffer, text, len);
    buf_ptr = buffer;
    entries = NULL;
    parse_error[0] = 0;
    int status = parse_buffer();
    if (status) hash = hash_field(hash, parse_error);
    while (entries) {
        struct tb_entry *e = entries;
        if (!status) {
            hash = hash_byte(hash, e->type);
            hash = hash_byte(hash, (unsigned char)e->level);
            hash = hash_byte(hash, (unsigned char)e->mnemonic);
            hash = hash_field(hash, e->title);
            hash = hash_field(hash, e->command);
        }
        entries = e->next;
        free(e);
    }
    free(buffer);
    return hash;
}
C
python3 - "$crate" "$original" "$work/inputs" <<'PYTHON'
import json, pathlib, sys
crate, original, inputs = map(pathlib.Path, sys.argv[1:])
texts = [(original / "toolboxrc").read_text(), (crate / "../../config/toolboxrc").read_text()]
for row in (crate / "tests/cases.jsonl").read_text().splitlines():
    texts.extend(item.partition("\0")[0] for item in json.loads(row)["files"].values())
texts.extend("A{\n" + "M{\n" * n + "L: run\n" + "}\n" * (n + 1) for n in (0, 1, 4, 16))
texts.extend("M{\n " + prefix + name + ":echo \"HELLO\"\\$HOME\n}"
             for prefix in ("&", "\\&", "\\\\", "") for name in ("Árvore", "x\\:y", "A&B", "App"))
inputs.write_text("".join(json.dumps({"fn": "menu_digest", "args": [s]}) + "\n" for s in texts))
PYTHON
"$tool" bridge "$work" menu.c
bridge="$work/rust/menu-rs"
"$tool" record "$bridge" --inputs "$work/inputs"
cp "$crate/src/parser.rs" "$bridge/src/parser.rs"
cat > "$bridge/src/port.rs" <<'RS'
#![forbid(unsafe_code)]
#[path = "parser.rs"]
mod parser;
fn byte(hash: u32, b: u8) -> u32 {
    (hash ^ u32::from(b)).wrapping_mul(16777619)
}
fn field(hash: u32, s: Option<&[u8]>) -> u32 {
    s.map_or_else(
        || byte(hash, 255),
        |s| byte(s.iter().copied().fold(hash, byte), 0),
    )
}
pub fn menu_digest(text: Option<&[u8]>) -> u32 {
    let Some(s) = text else {
        return 0;
    };
    let start = 2166136261;
    match parser::parse(s) {
        Err(error) => field(start, Some(error.as_bytes())),
        Ok(items) => items.into_iter().fold(start, |hash, e| {
            let hash = byte(byte(byte(hash, e.kind as u8), e.level as u8), e.mnemonic);
            let hash = field(
                hash,
                (e.kind != parser::Kind::Separator).then_some(e.title.as_slice()),
            );
            field(
                hash,
                (e.kind == parser::Kind::Command).then_some(e.command.as_slice()),
            )
        }),
    }
}
RS
cargo fmt --manifest-path "$bridge/Cargo.toml"
"$tool" build "$bridge"
"$tool" diff "$bridge"
"$tool" gate "$bridge"
