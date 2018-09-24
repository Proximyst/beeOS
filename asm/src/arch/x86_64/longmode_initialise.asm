  global long_mode_start

  section .text
  bits 64
  ;;   DESCRIPTION:
  ;; Starts to execute kernel code in 64 bit mode and initialises basic data.
  ;;
  ;; After execution, this halts the CPU.
  ;;
  ;;   LINKS:
  ;; The Rust part of the kernel for `rust_main`
long_mode_start:
  ;; Reload data segments for 64-bit use instead of 32 bit
  mov ax, 0
  mov ss, ax
  mov ds, ax
  mov es, ax
  mov fs, ax
  mov gs, ax

  ;; Starts the Rust part of the kernel.
  extern rust_main
  call rust_main

	hlt
