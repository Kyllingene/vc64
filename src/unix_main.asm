section .rodata

; color constants
BLACK:    dq 0x0
WHITE:    dq 0x1
GRAY:     dq 0x2
GREEN:    dq 0x3
BLUE:     dq 0x4
RED:      dq 0x5
DARKGRAY: dq 0x6

; key constants
KW:  dq 0x0
KA:  dq 0x1
KS:  dq 0x2
KD:  dq 0x3
KZ:  dq 0x4
KX:  dq 0x5
KC:  dq 0x6
KV:  dq 0x7
KSP: dq 0x8

; variables
section .data

PX: dq 20
PY: dq 20

; code
section .text
default rel

; declare all functions here
extern _draw_square ; x, y, w, c
extern _draw_rect   ; x, y, w, h, c
extern _draw_sprite ; x, y, *s
extern _clear       ; c
extern _key_down    ; k

global update

update:
	push rbp
	mov rbp, rsp
	
	mov rdi, [GRAY]
	call _clear
	
	mov rax, 0
	
move_up:
	mov rdi, qword [KW]
	call _key_down
	
	cmp rax, 0
	je move_down
	sub qword [PY], 1

move_down:
	mov rdi, qword [KS]
	call _key_down
	
	cmp rax, 0
	je move_left
	add qword [PY], 1

move_left:
	mov rdi, qword [KA]
	call _key_down
	
	cmp rax, 0
	je move_right
	sub qword [PX], 1

move_right:
	mov rdi, qword [KD]
	call _key_down
	
	cmp rax, 0
	je draw_player
	add qword [PX], 1
	
draw_player:
	cmp qword [PX], 0
	je fix_x
	jmp check_y
	
fix_x:
	mov qword [PX], 1
	
check_y:
	cmp qword [PY], 0
	je fix_y
	jmp coordinates_fixed

fix_y:
	mov qword [PY], 1

coordinates_fixed:
	mov rdi, 12
	mov rsi, 20
	mov rdx, 52
	mov rcx, 40
	mov r8, [BLUE]
	call _draw_rect
	
	mov rdi, [PX]
	mov rsi, [PY]
	lea rdx, [PLAYER]
	call _draw_sprite
	
	mov rsp, rbp
	pop rbp
	ret