%include "error_codes.asm"

  global check_cpuid

  section .text
  bits 32
  ;;   DESCRIPTION:
  ;; Checks for if the CPUID instruction is available on the system.
  ;;
  ;; If the instruction is unavailable, it will jump to $.cpuid_absent.
  ;; If it's present, it will `ret`.
check_cpuid:
  pushfd                        ; Push flags to stack
  pop eax                       ; Pop stack to eax

  mov ecx, eax                  ; Copy eax to ecx for later cmp

  xor eax, 1 << 21              ; Flip the 21st bit (CPUID bit)

  push eax                      ; Push back to stack
  popfd                         ; Return the flags back to CPU

  pushfd                        ; Get the flags back
  pop eax                       ; Pop the flags to eax, it shouldn't be ecx

  push ecx                      ; Restore ecx
  popfd                         ; Flip the bit back if it was flipped

  cmp eax, ecx                  ; Check if the flags changed when getting back
  je .cpuid_absent              ; Halt if the bit wasn't flipped
  ret                           ; Return if it was

  ;;   DESCRIPTION:
  ;; Halts the CPU with the error code "1".
  ;;
  ;;   LINKS:
  ;; `error.asm` for `error`
.cpuid_absent:
  CALL_ERROR(ERROR_CODE_NO_CPUID)
