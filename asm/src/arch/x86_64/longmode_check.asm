  global check_long_mode

  section .text
  bits 32
  ;;   DESCRIPTION:
  ;; Checks if the CPU has support for long mode.
  ;;
  ;; If it doesn't, jump to .long_mode_absent.
  ;; If it does, return to stack.
  ;;
  ;;   REFERENCES:
  ;; https://en.wikipedia.org/wiki/CPUID#EAX.3D80000001h:_Extended_Processor_Info_and_Feature_Bits
check_long_mode:
  mov eax, 0x80000000           ; Set `eax` to `0x80000000`
  cpuid                         ; Get highest supported argument into `eax`
  cmp eax, 0x80000001           ; Compare `eax` to `0x80000001`; does extended function exist?
  jb .long_mode_absent          ; Long mode doesn't have the ability to exist

  mov eax, 0x80000001           ; Set `eax` to `0x80000001`; argument for processor info
  cpuid                         ; Get info into `ecx` and `edx`
  test edx, 1 << 29             ; Check 29th bit, the long mode bit
  jz .long_mode_absent          ; If it's false, there is no long more support
  ret

  ;;   DESCRIPTION:
  ;; Hals the CPU with the error code "2".
  ;;
  ;;   LINKS:
  ;; `error.asm` for `error`
.long_mode_absent:
  mov al, "2"                   ; Move error "2" to `al`
  extern error                  ; Declare `error` for linking to `error.asm`
  jmp error                     ; Jump to `error` for halting
