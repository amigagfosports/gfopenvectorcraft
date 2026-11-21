/* AmigaDOS starts a LoadSeg executable at the first word of its first CODE
 * hunk; unlike ELF, a Hunk executable has no independent entry address.
 * Keep the Rust ABI bridge first, then follow the standard amiga linker
 * layout for code, read-only data, writable data and BSS. */
OUTPUT_FORMAT("amiga")
OUTPUT_ARCH(m68k)

SECTIONS
{
  PROVIDE(___machtype = 0x0);
  . = 0x0;

  .text :
  {
    __stext = .;
    KEEP(*(.text.startup))
    *(.text)
    *(.text.main)
    *(.text*)
    *(_*)
    *(.rodata*)
    *(.data.rel.ro*)
    *(.gnu.linkonce.t.*)
    *(.gnu.linkonce.r.*)
    *(.gcc_except_table*)
    __etext = .;
  }
  ___text_size = SIZEOF(.text);

  .data :
  {
    __sdata = .;
    *(.data)
    *(SORT_BY_NAME(.data.*))
    *(.gnu.linkonce.d.*)
    __edata = .;
  }
  ___data_size = SIZEOF(.data);

  .bss :
  {
    __bss_start = .;
    *(.bss)
    *(.bss.*)
    *(COMMON)
    __end = .;
  }
  ___bss_size = SIZEOF(.bss);

  .datachip : { *(.datachip) }
  .bsschip : { *(.bsschip) }

  /DISCARD/ : { *(.comment) *(.note*) }
}
