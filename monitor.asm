	.org 0
	; r1: Holds RX_DATA_FLAG
	; r2: Holds RX_DATA
	; r3: r2 temp
	; r4: Holds TX_BUSY
	; r5: Holds TX_DATA

	la r31, L1 ; Loop to wait for control character
	la r30, LQ ; '?' entrypoint
;	la r29, LP ; 'P' entrypoint
;	la r28, LW ; 'W' entrypoint
;	la r27, LR ; 'R' entrypoint
	
	la r26, Q0 ; 'R'
	la r25, Q1 ; 'I'
	la r24, Q2 ; 'C'
	la r23, Q3 ; 'H'

	



L1:	ld r1, 0xFFE8
	brzr r31, r1 ; Loop until RX_DATA_FLAG goes high

	ld r2, 0xFFEC ; Load RX_DATA into r2

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
	andi r3, r3, 0 ; Clear r3
	addi r3, r2, 0 ; Copy r2 into r3
	addi r3, r3, -82 ; Subtract ascii 'R'
	brzr r27, r3 ; Branch to LR if r3 is 0 (r2='R')


LQ:	nop
Q0:	ld r4, 0xFFE0 ; Read TX_BUSY into r4
	brnz r26, r4 ; Branch up if TX_BUSY = 1
	andi r5, r5, 0 ; Clear r5
	addi r5, r5, 82 ; Set r5 to 'R'
	st r5, 0xFFE4 ; Store r5 to TX_DATA

Q1:	ld r4, 0xFFE0 ; Read TX_BUSY into r4
	brnz r25, r4 ; Branch up if TX_BUSY = 1
	andi r5, r5, 0 ; Clear r5
	addi r5, r5, 73 ; Set r5 to 'I'
	st r5, 0xFFE4 ; Store r5 to TX_DATA

Q2:	ld r4, 0xFFE0 ; Read TX_BUSY into r4
	brnz r24, r4 ; Branch up if TX_BUSY = 1
	andi r5, r5, 0 ; Clear r5
	addi r5, r5, 67 ; Set r5 to 'C'
	st r5, 0xFFE4 ; Store r5 to TX_DATA

Q3:	ld r4, 0xFFE0   ; Read TX_BUSY into r4
	brnz r23, r4    ; Branch up if TX_BUSY = 1
	andi r5, r5, 0  ; Clear r5
	addi r5, r5, 72 ; Set r5 to 'H'
	st r5, 0xFFE4   ; Store r5 to TX_DATA

	br r31 ; Branch to beginning of program
	
