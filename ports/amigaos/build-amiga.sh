#!/bin/sh
# Copyright (c) 2026 AmiCradle Team. MIT OR Apache-2.0.
set -eu

port=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
stove=${STOVE:-/home/da1ek/AmiCradle/kitchen/stoves/os32}
out=${OUT:-$port/build-amiga}
target_dir=${CARGO_TARGET_DIR:-$port/target-kitchen}
prefix="$stove/prefix/bin"
rust_version=1.98.1

if ! command -v cargo >/dev/null 2>&1; then
    cache_root=${LUMAVAULT_TOOL_CACHE:-${XDG_CACHE_HOME:-${HOME:?HOME is required}/.cache}}
    rust_cache="$cache_root/gfopenvectorcraft-rust-$rust_version"
    export RUSTUP_HOME="$rust_cache/rustup"
    export CARGO_HOME="$rust_cache/cargo"
    export PATH="$CARGO_HOME/bin:$PATH"
    if [ ! -x "$CARGO_HOME/bin/cargo" ]; then
        mkdir -p "$rust_cache/download"
        rustup_init="$rust_cache/download/rustup-init"
        rustup_base=https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu
        curl -fsSL "$rustup_base/rustup-init" -o "$rustup_init"
        expected=$(curl -fsSL "$rustup_base/rustup-init.sha256" | awk 'NR == 1 { print $1 }')
        actual=$(sha256sum "$rustup_init" | awk '{ print $1 }')
        test -n "$expected"
        test "$actual" = "$expected"
        chmod 0755 "$rustup_init"
        "$rustup_init" -y --no-modify-path --profile minimal \
            --default-toolchain "$rust_version" \
            --component rust-src --component rustfmt --component clippy
    fi
fi

cargo --version
rustc --version

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
