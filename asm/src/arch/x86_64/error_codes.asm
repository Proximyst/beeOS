%ifndef __ERROR_CODES
%define __ERROR_CODES

  ;;    DESCRIPTION:
  ;; This error code indicates that multiboot2 didn't boot the kernel.
  ;; This is always a serious error and should be looked into if the user is able to.
%define ERROR_CODE_NO_MULTIBOOT "0"

  ;;   DESCRIPTION:
  ;; This indicates that the CPU is too old for the CPUID instruction.
  ;; If this happens, it's simply a case of it being too old and the user has to upgrade.
%define ERROR_CODE_NO_CPUID "1"

  ;;   DESCRIPTION:
  ;; This indicates that long mode isn't available on this CPU.
  ;; Long mode is required to use a 64-bit CPU.
  ;; If x86 is possible, rather compile and run that to use the kernel.
%define ERROR_CODE_NO_LONGMODE "2"

  ;;   DESCRIPTION:
  ;; Calls `error` from error.asm with the given error code.
%macro CALL_ERROR 1
  extern error
  mov al, %0
  call error
%endmacro

%endif