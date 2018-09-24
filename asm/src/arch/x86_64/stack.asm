  global stack_top
  global setup_page_tables
  global enable_paging

  section .bss
  bits 32
  align 4096
  ;;   DESCRIPTION:
  ;; The entirety of the paging mechanism is to have somewhere to store data.
  ;; These tables just define where to store the data.
  ;;
  ;;   REFERENCES:
  ;; http://pages.cs.wisc.edu/%7Eremzi/OSTEP/vm-paging.pdf
p4_table:
  resb 4096
p3_table:
  resb 4096
p2_table:
  resb 4096

  ;;   DESCRIPTION:
  ;; Reserve 16 KiB of stack memory.
  ;; This is used before heap is used in order to save time.
  ;;
  ;; On boot, `mov esp, stack_top` should be executed to set the stack.
  ;; The stack and registries are dependent on this.
  ;;
  ;; The top is used, even though it is under bottom in the code, because of the little-endianness of x86(_64).
stack_bottom:
  resb 4096 * 4                 ; Reserve 4 pages
stack_top:

  section .paging
  bits 32
  ;;   DESCRIPTION:
  ;; Sets up and maps the different tables for paging in long mode.
  ;; This is required to be done alongside the GDT, even though we mostly just use pages today.
  ;;
  ;;   REFERENCES:
  ;; http://pages.cs.wisc.edu/%7Eremzi/OSTEP/vm-paging.pdf
setup_page_tables:
  ;; Map first page 4 entry to page 4
  mov eax, p3_table
  or eax, 0b11                  ; Present and writable
  mov [p4_table], eax

  ;; Map first page 3 entry to page 2
  mov eax, p2_table
  or eax, 0b11                  ; Present and writable
  mov [p3_table], eax

  ;; Map each P2 entry to a 2MiB page
  mov ecx, 0                    ; Counter

.map_p2_table:
  mov eax, 0x200000             ; 2MiB
  mul ecx                       ; Get the starting addres of the ecx-th page
  or eax, 0b10000011            ; Present, writable, and huge
  mov [p2_table + ecx * 8], eax ; Map ecx-th entry

  inc ecx                       ; Increment counter
  cmp ecx, 512                  ; Use 512 P2 entries
  jne .map_p2_table             ; If less, jump back to map a new entry

  ret

  ;;   DESCRIPTION:
  ;; This enables the pages themselves, and must be called after the paging has been set up.
  ;; If that isn't done, this will not work properly and paging will be doing unexpected behaviour.
enable_paging:
  ;; Move page 4 to CR3, the CPU register for the P4 table
  mov eax, p4_table
  mov cr3, eax

  ;; Enable the physical address extension in CPU register 4
  mov eax, cr4
  or eax, 1 << 5
  mov cr4, eax

  ;; Set long mode with EFER model specific register
  mov ecx, 0xC0000080
  rdmsr                         ; Read MSR to eax with ecx given
  or eax, 1 << 8
  wrmsr                         ; Write eax to MSR

  ;; Enable paging
  mov eax, cr0
  or eax, 1 << 31
  mov cr0, eax

  ret
