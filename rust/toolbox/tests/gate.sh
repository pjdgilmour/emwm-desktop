#!/bin/sh
# Record the reference manual command parser's argument vectors before exec.
set -eu
original=$(realpath "$1")/src
crate=$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
tool=${RUST_ANYTHING:-rust-anything}
export XDG_CACHE_HOME="$work/cache"
python3 - "$original/tbmain.c" "$work" <<'PY'
import itertools, json, pathlib, sys
source = pathlib.Path(sys.argv[1]).read_text()
out = pathlib.Path(sys.argv[2])
begin = source.index('static int exec_command(const char *cmd_spec)')
end = source.index('pid = vfork();', begin)
function = source[begin:end].replace('static int exec_command(const char *cmd_spec)', 'unsigned command_digest(const char *cmd_spec)')
(out / 'command.c').write_text('''#include <stdlib.h>
#include <stdio.h>
#include <ctype.h>
#include <errno.h>
#include <string.h>
#include <sys/types.h>
''' + function + '''
    unsigned digest = 2166136261u;
    for (unsigned i = 0; i < argc; ++i) {
        const unsigned char *p = (const unsigned char *)argv[i];
        do { digest = (digest ^ *p) * 16777619u; } while (*p++);
    }
    free(argv);
    free(str);
    return digest;
}
''')
parts = ('a', ' ', '\t', "'", '"', '\\')
texts = [''.join(v) for n in range(4) for v in itertools.product(parts, repeat=n)]
texts.extend(('terminal -e "editor /tmp/test file"', "xterm -T 'Área de Trabalho' -e shell", ''))
(out / 'inputs').write_text(''.join(json.dumps({'fn':'command_digest','args':[s]}) + '\n' for s in texts))
PY
# The probe returns only a byte hash of argv, independent of pointer layout.
"$tool" bridge --force "$work" command.c
bridge="$work/rust/command-rs"
"$tool" record "$bridge" --inputs "$work/inputs"
cp "$crate/src/command_line.rs" "$bridge/src/command_line.rs"
cat >"$bridge/src/port.rs" <<'RS'
#![forbid(unsafe_code)]
#[path = "command_line.rs"]
mod command_line;
use std::os::unix::ffi::OsStrExt;
pub fn command_digest(input: Option<&[u8]>) -> u32 {
    let Ok(words) = command_line::split_command(input.unwrap_or_default()) else { return 22 };
    let mut digest = 2166136261;
    for word in words {
        for &b in word.as_bytes().iter().chain(std::iter::once(&0)) {
            digest = (digest ^ u32::from(b)).wrapping_mul(16777619);
        }
    }
    digest
}
RS
cargo fmt --manifest-path "$bridge/Cargo.toml"
"$tool" build "$bridge"
"$tool" diff "$bridge"
"$tool" gate "$bridge"
