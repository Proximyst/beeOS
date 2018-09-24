  global gdt64
  global gdt64.code_segment
  global gdt64.pointer

  section .rodata
  bits 32
  ;;   DESCRIPTION:
  ;; This is a GDT table. Code should be executed in the gdt64.code_segment block.
  ;; The top is at gdt64.pointer.
  ;;
  ;; I have no idea how this actually works, but it does, so let's leave it at that.
gdt64:
  dq 0
.code_segment: equ $ - gdt64
  dq (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53)
.pointer:
  dw $ - gdt64 - 1
  dq gdt64
