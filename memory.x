{% if board == "IskraJS" %}MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH (rx)   : ORIGIN = 0x08000000 + 32K, LENGTH = 1024K - 32K
  CCMRAM (xrw) : ORIGIN = 0x10000000, LENGTH = 64K
  RAM (xrw)    : ORIGIN = 0x20000000, LENGTH = 128K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

/* Advanced users can place the stack inthe CCMRAM */
/* which is smaller but faster. */
/* _stack_start = ORIGIN(CCMRAM) + LENGTH(CCMRAM); */

/* # Sections */
SECTIONS
{
  _siccmram = LOADADDR(.ccmram);

  /* CCM-RAM section 
  * 
  * IMPORTANT NOTE! 
  * If initialized variables will be placed in this section,
  * the startup code needs to be modified to copy the init-values.  
  */
  .ccmram ORIGIN(CCMRAM) :
  {
    . = ALIGN(4);
    _ccmram_start = .;       /* create a global symbol at ccmram start */
    *(.ccmram)
    *(.ccmram*)
    
    . = ALIGN(4);
    _ccmram_end = .;       /* create a global symbol at ccmram end */
  } >CCMRAM AT> FLASH
}
INSERT AFTER .rodata{% else %}MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH (rx)   : ORIGIN = 0x08000000 + 32K, LENGTH = 512K - 32K
  RAM (xrw)    : ORIGIN = 0x20000000, LENGTH = 128K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);{% endif %}
