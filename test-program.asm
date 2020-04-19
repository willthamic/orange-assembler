        .org 0 
        
        lar r2,MYVGA ; r2 must always be MYVGA, never MYVGAX! 
        la r4,BHALF 

        ld r1,MYD 
        not r1,r1 
        st r1,MYD 
        andi r1,r1,1 
        brnz r4,r1 

        la r1,0 ; Blank the top half of the screen 
        la r31,LOOP1 
        lar r30,MYVGA ; Use the base address 
LOOP1:  st r1,0(r30) 
        addi r30,r30,8 ; Write every other pixel
        and r3,r30,r2 
        brnz r31,r3 

        la r29,TOP 
        la r31,LOOP2 
TOP:    lar r30,MYVGA 
LOOP2:  ld r1,0(r30) 
        addi r1,r1,1 
        st r1,0(r30) 
        addi r30,r30,8 
        and r3,r30,r2 
        brnz r31,r3 
        br r29 
        stop

BHALF:  la r1,0 ; Blank the bottom half of the screen 
        la r31,LOOP1X 
        lar r30,MYVGAX ; Use the base address plus 4 
LOOP1X: st r1,0(r30) 
        addi r30,r30,8 ; Write every other pixel 
        and r3,r30,r2 
        brnz r31,r3 

        la r29,TOPX 
        la r31,LOOP2X 
TOPX:   lar r30,MYVGAX 
LOOP2X: ld r1,0(r30) 
        addi r1,r1,1 
        st r1,0(r30) 
        addi r30,r30,8 
        and r3,r30,r2 
        brnz r31,r3 
        br r29 
        stop 
        
        .org 4096
MYD:    .dw 1024 
        .org 2097152 
MYVGA:  .dw 524288 
        .org 2097156 
MYVGAX: .dw 524288 
