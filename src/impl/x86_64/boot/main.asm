; os entrypoint.


global start
extern long_mode_start

section .text
bits 32
start: 
    mov esp, stack_top

    call check_multiboot
    call check_cpuid
    call check_long_mode

    ; 64bit virtual memory is mapped here via paging. a page is 4KB of virtual mem that is mapped to 4KB of physical memory.

    call setup_page_tables
    call enable_paging

    lgdt [gdt64.pointer]
    jmp gdt64.code_segment:long_mode_start

    hlt

check_multiboot:
    cmp eax, 0x36d76289 ; compare eax register to multiboot 
    jne .no_multiboot ; if not equal? not multiboot
    ret
.no_multiboot:
    mov al, "M"
    jmp error

check_cpuid:
    pushfd
    pop eax ; store cpu info in eax register
    mov ecx, eax ; copy contents to ecx register
    xor eax, 1 << 21 ; toggle the 21st bit (if this does nothing, not 64bit)
    push eax
    popfd ; write modified eax back to EFlags register
    pushfd
    pop eax ; write EFlags back to eax (as modified or as original if not 64bit)
    push ecx
    popfd ; write original Eflags back to EFlags register from backed up ecx register (no system side-effects).
    cmp eax, ecx ; compare the original and modified registers
    je .no_cpuid ; if they are equal, system is NOT 64bit, jump to no cpu ID and provide system message to al
    ret
.no_cpuid:
    mov al, "C"
    jmp error ; essentially throw an error

check_long_mode:
    mov eax, 0x80000000
    cpuid
    cmp eax, 0x80000001
    jb .no_long_mode ; jump if eax register is less than max 64 bit number.

    mov eax, 0x80000001
    cpuid
    test edx, 1 << 29
    jz .no_long_mode ; jump if edx is zero
    
    ret
.no_long_mode:
    mov al, "L"
    jmp error

setup_page_tables:
    mov eax, page_table_l3
    or eax, 0b11 ; present writable
    mov [page_table_l4], eax

    mov eax, page_table_l2
    or eax, 0b11 ; present writable
    mov [page_table_l3], eax

    mov ecx, 0 ; counter
.loop:

    mov eax, 0x200000 ; 2MiB
    mul ecx
    or eax, 0b10000011 ; present, writable, huge page
    mov [page_table_l2 + ecx * 8], eax

    inc ecx ; increment counter
    cmp ecx, 512 ; check if whole table is mapped
    jne .loop ; if not, continue

    ret


enable_paging:
    ; pass page table location to cpu
    mov eax, page_table_l4
    mov cr3, eax

    ; enable PAE
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; enable long mode
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; enable paging
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    ret

error:
    ; print "ERR: X" where X is the error code
    mov dword [0xb8000], 0x4f524f45
    mov dword [0xb8004], 0x4f3a4f52
    mov dword [0xb8008], 0x4f204f20
    mov byte  [0xb800a], al

section .bss
align 4096
; four types of page tables, l4-l1. Each holds 502 entries
; first 9 bits of a virtual address are an index into the l4 page table
; corresponding entry will point to l3 page table. The next virtual address 9 bits index l3 page table.
; this continues through to l1 page table, where the entry points to a page in physical memory.
; the cpu determines the address of the l4 page table by reading the CR3 register.

; here, mem is reserved for page tables.
page_table_l4:
    resb 4096
page_table_l3:
    resb 4096
page_table_l2:
    resb 4096
stack_bottom:
    resb 4096 * 4
stack_top:

section .rodata
gdt64:
    dq 0 ; zero entry
.code_segment: equ $ - gdt64
    dq (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53) ; code segment
.pointer:
    dw $ - gdt64 - 1
    dq gdt64