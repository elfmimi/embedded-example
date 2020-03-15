/* NOTE: Adapted from cortex-m/link.x */
MEMORY
{
  RAM : ORIGIN = 0x80000000, LENGTH = 0x00080000
  STACK : ORIGIN = ORIGIN(RAM) + LENGTH(RAM), LENGTH = 0x800FFDC0 - ORIGIN(RAM) - LENGTH(RAM)
}

PROVIDE(_stack_start = ORIGIN(STACK) + LENGTH(STACK));

/* PROVIDE(trap_handler = default_trap_handler); */

/* # Pre-initialization function */
/* If the user overrides this using the `#[pre_init]` attribute or by creating a `__pre_init` function,
   then the function this points to will be called before the RAM is initialized. */
/* PROVIDE(__pre_init = default_pre_init); */

PROVIDE(abort = _abort);

SECTIONS
{
  PROVIDE(_stext = ORIGIN(RAM));

  .text ALIGN(_stext,4) :
  {
    /* initialize sp register */
    /* assuming _stack_start < 0x8_0000_0000 and _stack_start % 16 == 0 */
    LONG(((_stack_start >> 4) + 0x800) & 0xFFFFF000 | 0x137); /* lui sp, %hi(_sp) */
    LONG((_stack_start << (20 - 4)) & 0xFFF00000 | 0x10113); /* addi sp, sp, %lo(_sp) */
    LONG(0x00411113); /* slli sp, sp, 4 */
    /* assure pc register gets setup to the intended absolute address */
    LONG(((_start >> 2) + 0x800) & 0xFFFFF000 | 0x0B7); /* lui ra, %hi(_sp) */
    LONG((_start << (20 - 2)) & 0xFFF00000 | 0x08093); /* addi ra, ra, %lo(_sp) */
    LONG(0x00209093); /* slli ra, ra, 2 */
    LONG(0x00008067); /* jalr zero, 0(ra) == jr ra == ret */
    . = ALIGN(4);
    _start = .;
    KEEP(*(.init));
    *(.text .text.*);
    _exit = .;
    LONG(0x00100073); /* EBREAK == breakpoint */
    LONG(0x0000006F); /* jal zero, . == j . == infinite loop */
    _abort = .;
    LONG(0x00100073); /* EBREAK == breakpoint */
    LONG(0x0000006F); /* jal zero, . == j . == infinite loop */
  } > RAM

  .sdata ALIGN(8) :
  {
    *(.sdata .sdata.*);
    . = ALIGN(8);
  } > RAM

  .rodata ALIGN(8) :
  {
    *(.rodata .rodata.*);
    . = ALIGN(8);
  } > RAM

  .bss  ALIGN(8) :
  {
    _sbss = .;
    *(.bss .bss.*);
    . = ALIGN(8);
    _ebss = .;
  } > RAM

  .data : AT(LOADADDR(.rodata) + SIZEOF(.rodata))
  {
    _sidata = LOADADDR(.data);
    _sdata = .;
    /* Must be called __global_pointer$ for linker relaxations to work. */
    PROVIDE(__global_pointer$ = . + 0x800);
    *(.data .data.*);
    . = ALIGN(4);
    _edata = .;
  } > RAM

  PROVIDE(_heap_size = 0);

  /* fictitious region that represents the memory available for the heap */
  .heap (INFO) :
  {
    _sheap = .;
    . += _heap_size;
    . = ALIGN(4);
    _eheap = .;
  } > RAM

  /* fictitious region that represents the memory available for the stack */
/*
  .stack (INFO) :
  {
    _estack = .;
    . = _stack_start;
    _sstack = .;
  } > RAM
*/

  /* fake output .got section */
  /* Dynamic relocations are unsupported. This section is only used to detect
     relocatable code in the input files and raise an error if relocatable code
     is found */
  .got (INFO) :
  {
    KEEP(*(.got .got.*));
  }

  /* Discard .eh_frame, we are not doing unwind on panic so it is not needed */
  /DISCARD/ :
  {
    *(.eh_frame);
  }
}

/* Do not exceed this mark in the error messages below                | */
ASSERT(SIZEOF(.got) == 0, "
.got section detected in the input files. Dynamic relocations are not
supported. If you are linking to C code compiled using the `gcc` crate
then modify your build script to compile the C code _without_ the
-fPIC flag. See the documentation of the `gcc::Config.fpic` method for
details.");
