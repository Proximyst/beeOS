  global start

  section .text
  bits 32
  ;;   DESCRIPTION:
  ;; Starts the kernel and initialises all parts needed.
  ;; It may at any point error, though it shouldn't on a supported system (x86_64, CPUID supported, long mode, and pages work).
  ;;
  ;; If this errors, it will halt the CPU.
  ;;
  ;;   LINKS:
  ;; `stack.asm` for `stack_top`, `setup_page_tables`, and `enable_paging`
  ;; `gdt.asm` for `gdt64`
  ;; `longmode_check.asm` for `check_long_mode`
  ;; `longmode_initialise` for `long_mode_start`
  ;; `cpuid_check.asm` for `check_cpuid`
  ;; `error.asm` for `error`
start:
  ;; Create a stack
  extern stack_top
  mov esp, stack_top

  ;; Declare `error` for all users
  extern error

  ;; Verify that this was executed through multiboot2
  extern check_multiboot
  call check_multiboot

  ;; Verify that the `cpuid` instruction is available
  extern check_cpuid
  call check_cpuid

  ;; Verify that longmode exists
  extern check_long_mode
  call check_long_mode

  ;; Setup CPU memory paging
  extern setup_page_tables
  call setup_page_tables

  ;; Enable the paging
  extern enable_paging
  call enable_paging

  ;; Load the GDT
  extern gdt64.pointer
  lgdt [gdt64.pointer]

  ;; Move to a 64-bit code segment
  extern long_mode_start
  extern gdt64.code_segment
  jmp gdt64.code_segment:long_mode_start
