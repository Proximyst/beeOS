  global check_multiboot

  section .text
  bits 32
  ;;   DESCRIPTION:
  ;; Checks whether the kernel is loaded in multiboot mode, version 2.
  ;;
  ;; If it isn't, jump to .multiboot_absent.
  ;; If it is, return to stack.
  ;;
  ;;   VARIABLES:
  ;; `eax` - Given by GRUB2 once it starts the kernel
check_multiboot:
  cmp eax, 0x36d76289           ; Check for magic value
  jne .multiboot_absent         ; Jump if not true
  ret                           ; Return to stack

  ;;   DESCRIPTION:
  ;; Halts the CPU with the error code "0".
  ;;
  ;;   LINKS:
  ;; `error.asm` for `error`
.multiboot_absent:
  mov al, "0"                   ; Move error "0" to `al`
  extern error                  ; Declare `error` for linking to `error.asm`
  jmp error                     ; Jump to `error` for halting
