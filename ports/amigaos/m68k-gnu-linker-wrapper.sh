#!/usr/bin/env bash
# Copyright (c) 2026 AmiCradle Team. MIT OR Apache-2.0.
set -euo pipefail

linker=${M68K_REAL_LINKER:-m68k-linux-gnu-ld}
filtered=()
for argument in "$@"; do
    case $argument in
        -no-pie|-nodefaultlibs|-nostdlib)
            ;;
        -Wl,*)
            IFS=',' read -r -a forwarded <<< "${argument#-Wl,}"
            filtered+=("${forwarded[@]}")
            ;;
        *)
            filtered+=("$argument")
            ;;
    esac
done

exec "$linker" "${filtered[@]}"
