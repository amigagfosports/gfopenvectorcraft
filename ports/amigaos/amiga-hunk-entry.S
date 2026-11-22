/* AmigaDOS starts a LoadSeg executable at the first word of its first CODE
 * hunk.  vlink preserves ELF input-section order, while Rust's global
 * assembly startup section is not necessarily first after LTO.  Keep this
 * tiny, first input hunk as an explicit jump to the real runtime entry. */
    .section .text.hunk_entry,"ax",@progbits
    .globl __amiga_hunk_entry
    .type __amiga_hunk_entry,@function
__amiga_hunk_entry:
    jmp _start
    .size __amiga_hunk_entry, .-__amiga_hunk_entry
