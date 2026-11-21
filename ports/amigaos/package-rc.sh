#!/bin/sh
# Copyright (c) 2026 AmiCradle Team. MIT OR Apache-2.0.
set -eu

port=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$port/../.." && pwd)
tool_dir=${1:?usage: package-rc.sh /path/to/m68k-amigaos-bin}
name=GFOpenVectorCraft-0.1.0-rc1
dist="$root/dist/amigaos"
drawer="$dist/$name"

test -x "$tool_dir/m68k-amigaos-ld"
test -x "$tool_dir/m68k-amigaos-nm"

M68K_AMIGA_HUNK=1 \
M68K_REAL_LINKER="$tool_dir/m68k-amigaos-ld" \
M68K_NM="$tool_dir/m68k-amigaos-nm" \
M68K_LINK_TMPDIR="$port/target-hunk/link-tmp" \
CARGO_TARGET_DIR="$port/target-hunk" \
    "$port/check.sh"

mkdir -p "$drawer"
install -m 0755 "$port/target-hunk/m68k-unknown-none-elf/release/gfopenvectorcraft-amiga" "$drawer/VectorCraft"
install -m 0644 "$port/RELEASE-NOTES.txt" "$drawer/README.txt"

(cd "$dist" && lha aq "$name.lha" "$name")
(cd "$dist" && sha256sum "$name.lha" "$name/VectorCraft" "$name/README.txt" > SHA256SUMS)

echo "Release candidate: $dist/$name.lha"
