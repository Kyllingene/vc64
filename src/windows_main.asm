section .data
; colors
BLACK:    dq 0x0
WHITE:    dq 0x1
GRAY:     dq 0x2
GREEN:    dq 0x3
BLUE:     dq 0x4
RED:      dq 0x5
DARKGRAY: dq 0x6

; keys
KW:  dq 0x0
KA:  dq 0x1
KS:  dq 0x2
KD:  dq 0x3
KZ:  dq 0x4
KX:  dq 0x5
KC:  dq 0x6
KV:  dq 0x7
KSP: dq 0x8

; game data
PX: dq 20
PY: dq 20

section .text
default rel

; declare all functions here
extern _draw_square ; x, y, w, c
extern _draw_rect   ; x, y, w, h, c
extern _clear       ; c
extern _key_down    ; k

global update

update:
	push rbp
	mov rbp, rsp
	
	mov rcx, [GRAY]
	call _clear
	
	mov rax, 0
	
move_up:
	mov rcx, qword [KW]
	call _key_down
	
	cmp rax, 0
	je move_down
	sub qword [PY], 1

move_down:
	mov rcx, qword [KS]
	call _key_down
	
	cmp rax, 0
	je move_left
	add qword [PY], 1

move_left:
	mov rcx, qword [KA]
	call _key_down
	
	cmp rax, 0
	je move_right
	sub qword [PX], 1

move_right:
	mov rcx, qword [KD]
	call _key_down
	
	cmp rax, 0
	je draw_player
	add qword [PX], 1
	
draw_player:
	mov rcx, [PX]
	mov rdx, [PY]
	mov r8, 10
	mov r9, [GREEN]
	call _draw_square
	
	mov rcx, 10
	mov rdx, 20
	mov r8, 50
	mov r9, 30
	push qword [BLUE]
	sub rsp, 24
	call _draw_rect
	
	mov rsp, rbp
	pop rbp
	ret