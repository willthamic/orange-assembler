	.org 0
	;  r1: Holds RX_DATA_FLAG
	;  r2: Holds RX_DATA
	;  r3: RX_DATA temp
	;  r4: Holds TX_BUSY
	;  r5: Holds TX_DATA
	;  r6: TX_DATA temp

	;  r7: Scratch register
	;  r8: Program size
	;  r9: Program address increment

	la r31, L1 ; Loop to wait for control character
	la r30, LQ ; '?' entrypoint
	la r29, LP ; 'P' entrypoint
	la r28, LW ; 'W' entrypoint
	la r27, LR ; 'R' entrypoint

	la r26, LQ0 ; Moving loop address
	;  r25: Subroutine return address
	
;==========================;
; CONTROL SWITCH STATEMENT ;
;==========================;

L1:	ld r1, 0xFFFFFFE8 ; Load RX_DATA_FLAG into r1
	brzr r31, r1 ; Loop until RX_DATA_FLAG goes high

	ld r2, 0xFFFFFFEC ; Load RX_DATA into r2

	; CASE '?'
	andi r3, r3, 0 ; Clear r3
	addi r3, r2, 0 ; Copy r2 into r3
	addi r3, r3, -63 ; Subtract ascii '?'
	brzr r30, r3 ; Branch to LQ if r3 is 0 (r2='?')

	; CASE 'P'
	andi r3, r3, 0 ; Clear r3
	addi r3, r2, 0 ; Copy r2 into r3
	addi r3, r3, -80 ; Subtract ascii 'P'
	brzr r29, r3 ; Branch to LP if r3 is 0 (r2='P')

	; CASE 'W'
	andi r3, r3, 0 ; Clear r3
	addi r3, r2, 0 ; Copy r2 into r3
	addi r3, r3, -87 ; Subtract ascii 'W'
	brzr r28, r3 ; Branch to LW if r3 is 0 (r2='W')

	; CASE 'R'
	andi r3, r3, 0   ; Clear r3
	addi r3, r2, 0   ; Copy r2 into r3
	addi r3, r3, -82 ; Subtract ascii 'R'
	brzr r27, r3     ; Branch to LR if r3 is 0 (r2='R')

;=====================;
; QUESTION MARK LOGIC ;
;=====================;

LQ:	nop

	; Send 'R'
	la r26, LQ0     ; Update loop address
LQ0:	ld r4, 0xFFFFFFE0   ; Read TX_BUSY into r4
	brnz r26, r4    ; Branch up if TX_BUSY = 1
	andi r5, r5, 0  ; Clear r5
	addi r5, r5, 82 ; Set r5 to 'R'
	st r5, 0xFFFFFFE4   ; Store r5 to TX_DATA	

	; Send 'I'
	la r26, LQ1     ; Update loop address
LQ1:	ld r4, 0xFFFFFFE0   ; Read TX_BUSY into r4
	brnz r26, r4    ; Branch up if TX_BUSY = 1
	andi r5, r5, 0  ; Clear r5
	addi r5, r5, 73 ; Set r5 to 'I'
	st r5, 0xFFFFFFE4   ; Store r5 to TX_DATA

	; Send 'C'
	la r26, LQ2     ; Update loop address
LQ2:	ld r4, 0xFFFFFFE0   ; Read TX_BUSY into r4
	brnz r26, r4    ; Branch up if TX_BUSY = 1
	andi r5, r5, 0  ; Clear r5
	addi r5, r5, 67 ; Set r5 to 'C'
	st r5, 0xFFFFFFE4   ; Store r5 to TX_DATA
	
	; Send 'H'
	la r26, LQ3     ; Update loop address
LQ3:	ld r4, 0xFFFFFFE0   ; Read TX_BUSY into r4
	brnz r26, r4    ; Branch up if TX_BUSY = 1
	andi r5, r5, 0  ; Clear r5
	addi r5, r5, 72 ; Set r5 to 'H'
	st r5, 0xFFFFFFE4   ; Store r5 to TX_DATA
	
	; Send 'A'
	la r26, LQ4     ; Update loop address
LQ4:	ld r4, 0xFFFFFFE0   ; Read TX_BUSY into r4
	brnz r26, r4    ; Branch up if TX_BUSY = 1
	andi r5, r5, 0  ; Clear r5
	addi r5, r5, 65 ; Set r5 to 'A'
	st r5, 0xFFFFFFE4   ; Store r5 to TX_DATA
	
	; Send 'R'
	la r26, LQ5     ; Update loop address
LQ5:	ld r4, 0xFFFFFFE0   ; Read TX_BUSY into r4
	brnz r26, r4    ; Branch up if TX_BUSY = 1
	andi r5, r5, 0  ; Clear r5
	addi r5, r5, 82 ; Set r5 to 'R'
	st r5, 0xFFFFFFE4   ; Store r5 to TX_DATA
	
	; Send 'D'
	la r26, LQ6     ; Update loop address
LQ6:	ld r4, 0xFFFFFFE0   ; Read TX_BUSY into r4
	brnz r26, r4    ; Branch up if TX_BUSY = 1
	andi r5, r5, 0  ; Clear r5
	addi r5, r5, 68 ; Set r5 to 'D'
	st r5, 0xFFFFFFE4   ; Store r5 to TX_DATA
	
	; Send 'U'
	la r26, LQ7     ; Update loop address
LQ7:	ld r4, 0xFFFFFFE0   ; Read TX_BUSY into r4
	brnz r26, r4    ; Branch up if TX_BUSY = 1
	andi r5, r5, 0  ; Clear r5
	addi r5, r5, 85 ; Set r5 to 'U'
	st r5, 0xFFFFFFE4   ; Store r5 to TX_DATA
	
	; Send 'I'
	la r26, LQ8     ; Update loop address
LQ8:	ld r4, 0xFFFFFFE0   ; Read TX_BUSY into r4
	brnz r26, r4    ; Branch up if TX_BUSY = 1
	andi r5, r5, 0  ; Clear r5
	addi r5, r5, 73 ; Set r5 to 'I'
	st r5, 0xFFFFFFE4   ; Store r5 to TX_DATA
	
	; Send 'N'
	la r26, LQ9     ; Update loop address
LQ9:	ld r4, 0xFFFFFFE0   ; Read TX_BUSY into r4
	brnz r26, r4    ; Branch up if TX_BUSY = 1
	andi r5, r5, 0  ; Clear r5
	addi r5, r5, 78 ; Set r5 to 'N'
	st r5, 0xFFFFFFE4   ; Store r5 to TX_DATA
	
	; Send 'O'
	la r26, LQA     ; Update loop address
LQA:	ld r4, 0xFFFFFFE0   ; Read TX_BUSY into r4
	brnz r26, r4    ; Branch up if TX_BUSY = 1
	andi r5, r5, 0  ; Clear r5
	addi r5, r5, 79 ; Set r5 to 'O'
	st r5, 0xFFFFFFE4   ; Store r5 to TX_DATA
	
	; Send ' '
	la r26, LQB     ; Update loop address
LQB:	ld r4, 0xFFFFFFE0   ; Read TX_BUSY into r4
	brnz r26, r4    ; Branch up if TX_BUSY = 1
	andi r5, r5, 0  ; Clear r5
	addi r5, r5, 32 ; Set r5 to ' '
	st r5, 0xFFFFFFE4   ; Store r5 to TX_DATA
	
	; Send 'V'
	la r26, LQC     ; Update loop address
LQC:	ld r4, 0xFFFFFFE0   ; Read TX_BUSY into r4
	brnz r26, r4    ; Branch up if TX_BUSY = 1
	andi r5, r5, 0  ; Clear r5
	addi r5, r5, 86 ; Set r5 to 'V'
	st r5, 0xFFFFFFE4   ; Store r5 to TX_DATA
	
	; Send '2'
	la r26, LQD     ; Update loop address
LQD:	ld r4, 0xFFFFFFE0   ; Read TX_BUSY into r4
	brnz r26, r4    ; Branch up if TX_BUSY = 1
	andi r5, r5, 0  ; Clear r5
	addi r5, r5, 50 ; Set r5 to '2'
	st r5, 0xFFFFFFE4   ; Store r5 to TX_DATA

	br r31 ; Branch to beginning of program
	
;===============;
; PROGRAM LOGIC ;
;===============;

LP:	la r25, LP0 ; Set subroutine return address
	la r26, RX  ; Set r26 to RX
	br r26      ; Branch to RX subroutine

LP0:	la r8, 0       ; Clear r8
	addi r8, r3, 0 ; Copy r3 to r8
	la r9, 0       ; Clear r9
	addi r9, r9, -4 ; Set r9 to 0

LP1:	addi r9, r9, 4  ; Increment address
	la r25, LP2     ; Set subroutine return address
	la r26, RX      ; Set r26 to RX
	br r26          ; Branch to RX subroutine
LP2:	st r3, 4096(r9) ; Write read value into address
	sub r7, r8, r9  ; r7 = r8 - r9
	la r26, LP1     ; Set r26 to LP1
	brnz r26, r7    ; Branch to LP1 if not done writing program

	la r26, 4096
	br r26 ; Branch to beginning of loaded program

;=============;
; WRITE LOGIC ;
;=============;

LW:	la r25, LW0 ; Set subroutine return address
	la r26, RX  ; Set r26 to RX
	br r26      ; Branch to RX subroutine
	
LW0:	addi r7, r3, 0 ; Copy r3 into r7
	la r25, LW1 ; Set subroutine return address
	la r26, RX  ; Set r26 to RX
	br r26      ; Branch to RX subroutine

LW1:	st r3, 0(r7) ; Write value to address

	br r31 ; Branch to beginning of program

;============;
; READ LOGIC ;
;============;

LR:	la r25, LR0 ; Set subroutine return address
	la r26, RX  ; Set r26 to RX
	br r26      ; Branch to RX subroutine

LR0:	ld r6, 0(r3) ; Load from address into r6
	la r25, LR1 ; Set subroutine return address
	la r26, TX  ; Set r26 to TX
	br r26      ; Branch to TX subroutine

LR1:	br r31 ; Branch to beginning of program

;===============;
; RX SUBROUTINE ;
;===============;

RX:	andi r3, r3, 0 ; Clear r3
	
	la r26, RX0    ; Update loop address
RX0:	ld r1, 0xFFFFFFE8  ; Load RX_DATA_FLAG into r1
	brzr r26, r1   ; Loop until RX_DATA_FLAG goes high
	ld r2, 0xFFFFFFEC  ; Load RX_DATA into r2
	shl r2, r2, 24 ; Shift left 24 bits
	add r3, r3, r2 ; Add shifted RX_DATA into r3 

	la r26, RX1    ; Update loop address
RX1:	ld r1, 0xFFFFFFE8  ; Load RX_DATA_FLAG into r1
	brzr r26, r1   ; Loop until RX_DATA_FLAG goes high
	ld r2, 0xFFFFFFEC  ; Load RX_DATA into r2
	shl r2, r2, 16 ; Shift left 16 bits
	add r3, r3, r2 ; Add shifted RX_DATA into r3 

	la r26, RX2    ; Update loop address
RX2:	ld r1, 0xFFFFFFE8  ; Load RX_DATA_FLAG into r1
	brzr r26, r1   ; Loop until RX_DATA_FLAG goes high
	ld r2, 0xFFFFFFEC  ; Load RX_DATA into r2
	shl r2, r2, 8  ; Shift left 8 bits
	add r3, r3, r2 ; Add shifted RX_DATA into r3 

	la r26, RX3    ; Update loop address
RX3:	ld r1, 0xFFFFFFE8  ; Load RX_DATA_FLAG into r1
	brzr r26, r1   ; Loop until RX_DATA_FLAG goes high
	ld r2, 0xFFFFFFEC  ; Load RX_DATA into r2
	add r3, r3, r2 ; Add shifted RX_DATA into r3 

	br r25 ; Branch out of subroutine

;===============;
; TX SUBROUTINE ;
;===============;

TX:	nop

	andi r7, r6, 0xFF000000 ; Copy first byte into scratch register
	shr r7, r7, 24      ; Shift right by 24
	la r26, TX0         ; Update loop address
TX0:	ld r4, 0xFFFFFFE0       ; Read TX_BUSY into r4
	brnz r26, r4        ; Branch up if TX_BUSY = 1
	st r7, 0xFFFFFFE4       ; Store r7 to TX_DATA

	andi r7, r6, 0x00FF0000 ; Copy second byte into scratch register
	shr r7, r7, 16      ; Shift right by 16
	la r26, TX1         ; Update loop address
TX1:	ld r4, 0xFFFFFFE0       ; Read TX_BUSY into r4
	brnz r26, r4        ; Branch up if TX_BUSY = 1
	st r7, 0xFFFFFFE4       ; Store r7 to TX_DATA

	andi r7, r6, 0x0000FF00 ; Copy third byte into scratch register
	shr r7, r7, 8       ; Shift right by 8
	la r26, TX2         ; Update loop address
TX2:	ld r4, 0xFFFFFFE0       ; Read TX_BUSY into r4
	brnz r26, r4        ; Branch up if TX_BUSY = 1
	st r7, 0xFFFFFFE4       ; Store r7 to TX_DATA

	andi r7, r6, 0x000000FF ; Copy fourth byte into scratch register
	la r26, TX3         ; Update loop address
TX3:	ld r4, 0xFFFFFFE0       ; Read TX_BUSY into r4
	brnz r26, r4        ; Branch up if TX_BUSY = 1
	st r7, 0xFFFFFFE4       ; Store r7 to TX_DATA

	br r25 ; Branch out of subroutine
