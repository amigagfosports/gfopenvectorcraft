#!/usr/bin/env bash
# Copyright (c) 2026 AmiCradle Team. MIT OR Apache-2.0.
set -euo pipefail

linker=${M68K_REAL_LINKER:-m68k-linux-gnu-ld}
hunk=${M68K_AMIGA_HUNK:-0}
script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
vlink_hunk=0
if [[ $hunk == 1 && $(basename -- "$linker") == vlink ]]; then
    vlink_hunk=1
fi
filtered=()
if [[ $hunk == 1 && $vlink_hunk == 0 ]]; then
    filtered+=("-T" "$script_dir/amiga-hunk.ld")
elif [[ $vlink_hunk == 1 ]]; then
    filtered+=("-b" "amigahunk" "-Rshort" "-k")
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
                    1:--gc-sections)
                        if [[ $vlink_hunk == 1 ]]; then
                            filtered+=("-gc-all")
                        else
                            filtered+=("$option")
                        fi
                        ;;
                    1:--strip-debug)
                        if [[ $vlink_hunk == 1 ]]; then
                            filtered+=("-S")
                        else
                            filtered+=("$option")
                        fi
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

if [[ $hunk == 1 && $vlink_hunk == 0 ]]; then
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

if [[ $vlink_hunk == 1 ]]; then
    # AmigaDOS has no independent entry-address field: execution starts at
    # byte zero of the first CODE hunk.  Rust/LTO may put another .text
    # section before .text.startup, so prepend a six-byte relocation-safe
    # jump to the runtime's _start symbol.
    link_tmp_root=${M68K_LINK_TMPDIR:-${TMPDIR:-/tmp}}
    mkdir -p -- "$link_tmp_root"
    temporary=$(mktemp -d "$link_tmp_root/m68k-vlink-entry.XXXXXX")
    trap 'rm -rf -- "$temporary"' EXIT
    assembler=${M68K_AS:-m68k-linux-gnu-as}
    "$assembler" -m68040 -o "$temporary/amiga-hunk-entry.o" \
        "$script_dir/amiga-hunk-entry.S"
    filtered=("$temporary/amiga-hunk-entry.o" "${filtered[@]}" \
        "-e" "__amiga_hunk_entry")
    "$linker" "${filtered[@]}"
    exit $?
fi

exec "$linker" "${filtered[@]}"
