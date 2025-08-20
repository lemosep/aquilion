[org 0x7c00]

	mov bp, 0x9000			; set the stack
	mov sp, bp

	mov bx, MSG_REAL_MODE
	call print_string

	jmp $

%include "print_string_rm.asm"

; Global variables
	MSG_REAL_MODE:
		db "Started in 16 bit real mode", 0, 13, 10

; Bootsector padding
times 510-($-$$) db 0
dw 0xAA55		; Magic number for boot sector
