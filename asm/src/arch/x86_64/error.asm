  global error

  section .text
  bits 32
  ;;   DESCRIPTION:
  ;; Prints "ERR: " to the screen together with the error code in `al`.
  ;; The text is all white on a red background.
  ;;
  ;; This halts the CPU.
  ;;
  ;;   VARIABLES:
  ;; `al` - Error code in a single byte.
error:
  ;; 0x4f = red background on white text
  mov dword [0xb8000], 0x4f524f45 ; "R" then "E" (little endian) - white text, red background
  mov dword [0xb8004], 0x4f3a4f52 ; ":" then "R" (little endian) - white text, red background
  mov dword [0xb8008], 0x4f204f20 ; " " then " " (little endian) - white text, red background
  ;; Write `al` to the byte right after the first space shown.
  ;; The other space is still red and white, making this byte red and white.
  mov  byte [0xb800a], al
  hlt                           ; Halt the CPU
