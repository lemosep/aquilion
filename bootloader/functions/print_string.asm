; Function that prints a string stored in bx register - This function is 
; used only in real mode 16-bit, as it uses BIOS functions.

print_string:
	mov ah, 0x0E		; BIOS teletype output

.next_char:
	mov al, [bx]
	cmp al, 0
	je .end_print_string
	
	int 0x10
	
	inc bx
	jmp .next_char

.end_print_string:
	ret
