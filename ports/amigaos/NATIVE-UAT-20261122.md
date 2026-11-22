# VectorCraft native 68k UAT — 22 November 2026

This receipt qualifies the AmigaOS port at Forge commit
`fa4b2a69f44207629747adce6ee3f3bdd185de93`. It does not qualify the full
upstream desktop application.

## Candidate identity

- repository: `amigagfosports/gfopenvectorcraft`
- branch: `team1/amigaos68k-foundation-20261120`
- GFOpenRust dependency: `7a3e3e1523e6233311fc30dc2f5a8bc51f6291e5`
- local os32 plate executable: AmigaOS LoadSeg Hunk, 35,664 bytes
- executable SHA-256:
  `174c222123ed0b33594d84e2f7a1a74871c4df06c4d27a8870953f11ff3c6fcb`

## Build gates run in this session

- `cargo fmt --all --check`: passed
- host unit tests: 8 passed, 0 failed
- `cargo clippy --all-targets -- -D warnings`: passed
- os32 `vlink` Hunk build and symbol gate: passed
- plate staging by `ports/amigaos/build-amiga.sh`: passed

Rust emitted only its two compiler notices for the deliberately selected
experimental `isa-68881` and `isa-68882` target features. The port itself
emitted no release-build warnings.

## Native lab

- lab: `t1-vectorcraft-20261120`
- source: AmiCradle Instance 52
- engine: AC090 native
- guest: AmigaOS-compatible 68k Workbench
- target: 68040/FPU

The previous executable was archived before installing the candidate. The
installed file was checksummed again as the candidate SHA above, then the lab
was reset for a cold start.

### Persistence and restart

1. The candidate opened its native GFOpenLayout editor after cold boot.
2. `Open` read the existing 446-byte `SYS:VectorCraft.vcf`.
3. The canvas restored all five saved objects and reported
   `SYS:VectorCraft.vcf opened`.

The saved file SHA-256 was
`54aa15a237a2a52f9ca57be2705953a82f113a6e29c5d1f9f604fc59b9824bd7`.

### Malformed-file recovery

1. The valid document was archived under
   `SYS:Tests/VectorCraft/fa4b2a69f-uat/`.
2. A three-byte truncated file was installed as `SYS:VectorCraft.vcf`.
3. `New` made a blank document.
4. `Open` rejected the truncated input with `Open failed: magic` and
   `VCF1 parser rejected a bounded field`.
5. The blank canvas remained intact and the application stayed responsive.
6. The exact valid document was restored and checksummed.
7. `Open` immediately restored the five objects again.

The malformed fixture SHA-256 was
`8c197e8e71b3752430f941cd9d031262faf9bdef82534d703b1e1c3478670d68`.

## Kitchen status

The LumaVault and Cradle Kitchen calls both reached the shared checkout path,
but the first full VectorCraft checkout (about 709 MB) exceeded the MCP call's
timeout. The Forge mirror and branch resolved correctly. The checkout was
restarted non-destructively to warm the clean Kitchen cache; a Forge-cooked
plate receipt remains required before merge or release promotion.
