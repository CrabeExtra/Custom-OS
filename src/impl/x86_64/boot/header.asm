; this lets bootloaders know the OS is here to be loaded to the computer. This will follow the multiboot specs.
header_start:
    ; magic #
    dd 0xe85250d6
    ; architecture 
    dd 0 ; protected mode i386
    ; header length
    dd header_end - header_start
    ; checksum
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))
    ;end tag
    dw 0
    dw 0
    dd 8
header_end: