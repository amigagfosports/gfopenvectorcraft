#!/usr/bin/env bash
# Copyright (c) 2026 AmiCradle Team. MIT OR Apache-2.0.
set -euo pipefail

linker=${M68K_REAL_LINKER:-m68k-linux-gnu-ld}
hunk=${M68K_AMIGA_HUNK:-0}
filtered=()
if [[ $hunk == 1 ]]; then
    script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
    filtered+=("-T" "$script_dir/amiga-hunk.ld")
fi
for argument in "$@"; do
    case $argument in
        -no-pie|-nodefaultlibs|-nostdlib)
            ;;
        -Wl,*)
            IFS=',' read -r -a forwarded <<< "${argument#-Wl,}"
            for option in "${forwarded[@]}"; do
                case $hunk:$option in
                    1:--as-needed|1:-Bstatic|1:-Bdynamic|1:--eh-frame-hdr|1:-z|1:noexecstack)
                        ;;
                    *)
                        filtered+=("$option")
                        ;;
                esac
            done
            ;;
        *)
            filtered+=("$argument")
            ;;
    esac
done

if [[ $hunk == 1 ]]; then
    # The Amiga BFD backend only emits LoadSeg relocation hunks when its input
    # objects are Amiga Hunk objects.  Rust produces ELF relocatables, so make
    # temporary, equivalent Hunk objects (and archives) before the final link.
    # Without this conversion the binary looks like a Hunk but every absolute
    # address is left relative to zero and it traps as soon as AmigaDOS runs it.
    link_tmp_root=${M68K_LINK_TMPDIR:-${TMPDIR:-/tmp}}
    mkdir -p -- "$link_tmp_root"
    temporary=$(mktemp -d "$link_tmp_root/m68k-hunk-link.XXXXXX")
    trap 'rm -rf -- "$temporary"' EXIT
    tool_dir=$(dirname -- "$linker")
    objcopy="$tool_dir/m68k-amigaos-objcopy"
    archive_tool="$tool_dir/m68k-amigaos-ar"
    converted=()
    index=0
    for argument in "${filtered[@]}"; do
        case $argument in
            *.o)
                output="$temporary/object-$index.o"
                "$objcopy" -O amiga "$argument" "$output"
                converted+=("$output")
                ;;
            *.a|*.rlib)
                archive_dir="$temporary/archive-$index"
                mkdir -p "$archive_dir"
                (
                    cd "$archive_dir"
                    "$archive_tool" x "$argument"
                    members=()
                    member_index=0
                    for member in *; do
                        if file "$member" | grep -q 'ELF .* relocatable'; then
                            hunk_member="member-$member_index.o"
                            "$objcopy" -O amiga "$member" "$hunk_member"
                            members+=("$hunk_member")
                            member_index=$((member_index + 1))
                        fi
                    done
                    "$archive_tool" crs converted.a "${members[@]}"
                )
                converted+=("$archive_dir/converted.a")
                ;;
            *)
                converted+=("$argument")
                ;;
        esac
        index=$((index + 1))
    done
    filtered=("${converted[@]}" "--emit-relocs")
fi

exec "$linker" "${filtered[@]}"
