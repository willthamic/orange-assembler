	.org 0
	la r31,LOOP
        la r29,TOP
	la r1,0
	lar r2,2097152
TOP:	lar r30,MYVGA
LOOP:	st r1,0(r30)
	addi r30,r30,4
	and r3,r30,r2
	brnz r31,r3
	addi r1,r1,1
	ld r28,-24(r0) ; Read RX_DATA_FLAG
	brzr r29,r28
	ld r28,-20(r0) ; Read RX_DATA
	st r28,-28(r0) ; Store to TX_DATA, no need to test TX_BUSY
        br r29
	stop

	.org 4096 ;
MYD:	.dw 1024

	.org 2097152
MYVGA:	.dw 524288
