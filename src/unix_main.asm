section .data
; sprites (i used the sprite editor from http://johnearnest.github.io/Octo/index.html)
PLAYER: db 0x3F, 0xFC, 0x7F, 0xFE, 0xF0, 0x0F, 0xE0, 0x07, 0xE0, 0x07, 0xC4, 0x23, 0xC4, 0x23, 0xC0, 0x03, 0xC0, 0x03, 0xC8, 0x13, 0xC7, 0xE3, 0xE0, 0x07, 0xE0, 0x07, 0xF0, 0x0F, 0x7F, 0xFE, 0x3F, 0xFC

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
	mov rdi, 10
	mov rsi, 20
	mov rdx, 50
	mov rcx, 30
	mov r8, [BLUE]
	call _draw_rect
	
	mov rdi, [PX]
	mov rsi, [PY]
	lea rdx, [PLAYER]
	call _draw_sprite
	
	mov rsp, rbp
	pop rbp
	ret