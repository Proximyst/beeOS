ENTRY(start)

SECTIONS
{
  /* Load in the first megabyte of RAM */
  . = 1M;

  /* Ensure the header is first */
  .boot : {
    /* KEEP is used due to making sure that it won't be removed if
       the optimiser is used to remove unused symbols */
    KEEP(*(.multiboot_header))
  }

  /* Actually link the .text section, and that before initialised memory is */
  .text : {
    *(.text)
  }
}
