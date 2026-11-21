# GFOpenVectorCraft for AmigaOS 3.x

GFOpenVectorCraft is a native AmigaOS 68k application, not a host wrapper. The
release build is an Amiga Hunk executable for a 68040 system and uses
GFOpenLayout for its Intuition interface.

## Candidate scope

The native editor window includes a toolbar, tools, canvas preview, layers,
properties and appearance panels. The working commands are:

- New document;
- Add Rectangle, Add Ellipse and Add Label;
- Undo and Redo;
- Select and Preview mode;
- native window close and clean AmigaDOS return (release gate; native proof required).

Each editing command changes the bounded Rust document state and refreshes the
native canvas, layer list, Undo/Redo controls, status line and window title.
Open, Save and interactive zoom are deliberately disabled and labelled as RC2
work.

The packaging script creates an RC-shaped test archive, but the archive must
not be published or promoted until native pointer actions and close have both
passed in an isolated 68040 lab. This candidate does not claim the upstream egui interface, complete
`.vectorcraft` file compatibility, OpenGPU rendering, text effects or file I/O.
It is a usable native interaction slice and a stable base for those features.

## Build gates

`ports/amigaos/check.sh` always runs formatting, host tests and clippy. With the
normal GNU m68k tools it also checks the freestanding ELF link used by CI.
With an Amiga-capable binutils prefix it builds and inspects the release Hunk:

```sh
M68K_AMIGA_HUNK=1 \
M68K_REAL_LINKER=/path/to/bin/m68k-amigaos-ld \
M68K_NM=/path/to/bin/m68k-amigaos-nm \
ports/amigaos/check.sh
```

`ports/amigaos/package-rc.sh /path/to/bin` produces the install drawer, LHA
archive and SHA-256 manifest under `dist/amigaos/`.

The port is MIT OR Apache-2.0, matching VectorCraft upstream. Its GFOpenRust
dependencies are pinned by full commit in `Cargo.toml` and `Cargo.lock`.
