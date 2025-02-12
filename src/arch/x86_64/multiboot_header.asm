section .multiboot_header
header_start:
    dd 0xE85250D6   ; Magic number
    dd 0
    dd header_end - header_start
    ; Checksum
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    ; End tag
    dw 0
    dw 0
    dd 8
header_end: