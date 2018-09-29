%include "error_codes.asm"

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
	; Check for magic value assigned by a multiboot compatible bootloader
  cmp eax, 0x36d76289

	; Jump if it isn't equal to the one it should be
  jne .multiboot_absent
  ret

  ;;   DESCRIPTION:
  ;; Halts the CPU with the error code `error_code_no_multiboot` from `error_codes.asm`.
  ;;
  ;;   LINKS:
  ;; `error.asm` for `error`
.multiboot_absent:
	; Move the error code to `al`
  CALL_ERROR(ERROR_CODE_NO_MULTIBOOT)

	; Declare `error` for linking to `error.asm`
  extern error
	; Jump to `error` for halting the CPU
  jmp error
