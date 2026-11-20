# GFOpenVectorCraft for AmigaOS 3.x

This is a native 68040 port boundary, not a host wrapper. It is a standalone
Cargo workspace while VectorCraft's portable crates remain `std`-bound.

The current slice provides:

- AmigaDOS startup and an Exec-pool allocator from GFOpenRust, pinned by commit;
- a bounded, fallible native subset for documents, artboards and paths with
  stable ids and finite-coordinate validation;
- explicit future service boundaries for GFOpenLayout/OpenGadTools,
  OpenRTG/OpenGPU, GFOpenDataTypes and GFOpenUp;
- a static big-endian m68k ELF with no unresolved symbols or relocations.

Run the current gate with `ports/amigaos/check.sh`.

This does not claim the egui interface, full `.vectorcraft` format, rendering,
text, effects or file I/O are ported. The model is an adapter subset, not a
second document format. The next slice is the native GFOpenLayout window and
canvas boundary, followed by OpenRTG rendering.
