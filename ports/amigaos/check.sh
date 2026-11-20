#!/bin/sh
# Copyright (c) 2026 AmiCradle Team. MIT OR Apache-2.0.
set -eu

port=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
manifest="$port/Cargo.toml"
linker=${M68K_LINKER:-$port/m68k-gnu-linker-wrapper.sh}
nm=${M68K_NM:-m68k-linux-gnu-nm}
readelf=${M68K_READELF:-m68k-linux-gnu-readelf}

cargo fmt --manifest-path "$manifest" --all --check
cargo test --manifest-path "$manifest"
cargo clippy --manifest-path "$manifest" --all-targets -- -D warnings

RUSTC_BOOTSTRAP=1 \
RUSTFLAGS="-C target-cpu=M68040 -C target-feature=-isa-68881,-isa-68882 -C code-model=large -C linker=${linker} -C link-arg=-nostdlib -C link-arg=-Wl,-e,_start" \
    cargo build --manifest-path "$manifest" --release \
    --target m68k-unknown-none-elf \
    -Zbuild-std=core,alloc,compiler_builtins \
    -Zbuild-std-features=compiler-builtins-mem

program="$port/target/m68k-unknown-none-elf/release/gfopenvectorcraft-amiga"
file "$program" | grep -Eq 'ELF 32-bit MSB executable, Motorola m68k.*statically linked'
test -z "$("$nm" -u "$program")"
for symbol in _start gf_amiga_main gf_lib_call0 gf_dos_call_d1_d2_d3; do
    "$nm" -g --defined-only "$program" | grep -Eq "[[:space:]]${symbol}$"
done
"$readelf" -rW "$program" | grep -Fq 'There are no relocations in this file.'

echo "GFOpenVectorCraft AmigaOS foundation passed: $program"
