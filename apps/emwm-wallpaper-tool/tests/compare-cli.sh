#!/bin/sh
# Record the C++ CLI and compare the Rust binary in isolated directories.
set -eu
original=$1
ported=$2
inputs=$3
fixture=$4
tool=$5
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
mkdir "$work/cases"
"$tool" app record "$work/cases" --name emwm-wallpaper-tool --no-traps --inputs "$inputs" -- "$original"
"$tool" app replay "$work/cases" --build "$ported"
# Both runs use the same absolute HOME because the Xresources include contains
# that path. A recording of the requested xrdb arguments also checks the merge.
mkdir "$work/bin" "$work/expected"
printf '%s\n' '#!/bin/sh' 'printf "%s\n" "$@" >> "$HOME/merge.args"' > "$work/bin/xrdb"
chmod u+x "$work/bin/xrdb"
for binary in "$original" "$ported"; do
    mkdir "$work/home"
    "$fixture" fixture "$work/home"
    HOME="$work/home" PATH="$work/bin" "$binary" --migrate > "$work/home/result"
    HOME="$work/home" PATH="$work/bin" "$binary" --migrate >> "$work/home/result"
    if [ "$binary" = "$original" ]; then
        mv "$work/home" "$work/expected/home"
    else
        diff -r "$work/expected/home" "$work/home"
    fi
done
