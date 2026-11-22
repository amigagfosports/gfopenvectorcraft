#!/bin/sh
# Copyright (c) 2026 AmiCradle Team. MIT OR Apache-2.0.
set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/.." && pwd)
out=${OUT:-"$root/build-amiga-package"}
version=0.1.0-rc2
archive="GFOpenVectorCraft-${version}-68040"
stage=$(mktemp -d "${TMPDIR:-/tmp}/gfopenvectorcraft-package.XXXXXX")
trap 'rm -rf "$stage"' EXIT HUP INT TERM

mkdir -p "$out" "$stage/$archive/VectorCraft"
OUT="$stage/plate" "$root/ports/amigaos/build-amiga.sh"

install -m 0755 "$stage/plate/VectorCraft" "$stage/$archive/VectorCraft/VectorCraft"
install -m 0644 "$stage/plate/README.txt" "$stage/$archive/VectorCraft/README.txt"
install -m 0644 "$root/LICENSE-MIT" "$stage/$archive/VectorCraft/LICENSE-MIT"
install -m 0644 "$root/LICENSE-APACHE" "$stage/$archive/VectorCraft/LICENSE-APACHE"
install -m 0644 "$root/NOTICE" "$stage/$archive/VectorCraft/NOTICE"
install -m 0644 "$root/ATTRIBUTION.md" "$stage/$archive/VectorCraft/ATTRIBUTION.md"
install -m 0644 "$here/generated/icons/VectorCraft.info" "$stage/$archive/VectorCraft/VectorCraft.info"
install -m 0644 "$here/generated/icons/README.txt.info" "$stage/$archive/VectorCraft/README.txt.info"
install -m 0644 "$here/generated/icons/VectorCraftDrawer.info" "$stage/$archive/VectorCraft.info"
install -m 0644 "$here/generated/installer/Install-VectorCraft" "$stage/$archive/Install-VectorCraft"
install -m 0644 "$here/generated/installer/Install-VectorCraft.info" "$stage/$archive/Install-VectorCraft.info"

(cd "$stage" && lha aq "$archive.lha" "$archive")
install -m 0644 "$stage/$archive.lha" "$out/$archive.lha"
(cd "$out" && sha256sum "$archive.lha" > "$archive.lha.sha256")
install -m 0644 "$root/ports/amigaos/RELEASE-NOTES.txt" "$out/$archive.readme"

echo "GFOpenVectorCraft RC package: $out/$archive.lha"
