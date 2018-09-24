  section .multiboot_header
  bits 32

  ;;   DESCRIPTION:
  ;; Header for the multiboot mode, version 2 protocol.
  ;; All multiboot2 compatible bootloaders check for this in the kernel before boot.
header_start:
  ;; Multiboot 2 requires a magic number to be start
  ;; That is the following DWORD:
	dd 0xe85250d6

  ;; The architecture `protected i368` is `0` and it must be provided in a DWORD:
  dd 0

  ;; The length of the rest of the header should be provided as a DWORD:
	dd header_end - header_start

  ;; A checksum of the header should exist and should be a DWORD:
	dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

  ;; Multiboot then wants us to end with some other magic values:
  dw 0
  dw 0
  dd 8
header_end:
