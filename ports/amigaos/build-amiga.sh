#!/bin/sh
# Copyright (c) 2026 AmiCradle Team. MIT OR Apache-2.0.
set -eu

port=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
stove=${STOVE:-/home/da1ek/AmiCradle/kitchen/stoves/os32}
out=${OUT:-$port/build-amiga}
target_dir=${CARGO_TARGET_DIR:-$port/target-kitchen}
prefix="$stove/prefix/bin"

test -x "$prefix/vlink"
test -x "$prefix/m68k-amigaos-as"
test -x "$prefix/m68k-amigaos-nm"
mkdir -p "$out" "$target_dir/link-tmp"
out=$(CDPATH= cd -- "$out" && pwd)

M68K_AMIGA_HUNK=1 \
M68K_REAL_LINKER="$prefix/vlink" \
M68K_AS="$prefix/m68k-amigaos-as" \
M68K_NM="$prefix/m68k-amigaos-nm" \
M68K_LINK_TMPDIR="$target_dir/link-tmp" \
CARGO_TARGET_DIR="$target_dir" \
    "$port/check.sh"

program="$target_dir/m68k-unknown-none-elf/release/gfopenvectorcraft-amiga"
install -m 0755 "$program" "$out/VectorCraft"
install -m 0644 "$port/RELEASE-NOTES.txt" "$out/README.txt"
(cd "$out" && sha256sum VectorCraft README.txt > SHA256SUMS)

echo "VectorCraft plate staged in $out"
