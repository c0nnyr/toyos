    .section .text
restore_trap_asm:
    # fn (ptr *TrapContext)
    # a0=x10=ptr
    mv x31, x10 # t6=x31
    ld x30, 32*8(x31) #t5=x30
    csrw sepc, x30
    ld x1, 1*8(x31); ld x2, 2*8(x31); ld x3, 3*8(x31); ld x4, 4*8(x31); ld x5, 5*8(x31); ld x6, 6*8(x31); ld x7, 7*8(x31); ld x8, 8*8(x31); ld x9, 9*8(x31); ld x10, 10*8(x31); ld x11, 11*8(x31); ld x12, 12*8(x31); ld x13, 13*8(x31); ld x14, 14*8(x31); ld x15, 15*8(x31); ld x16, 16*8(x31); ld x17, 17*8(x31); ld x18, 18*8(x31); ld x19, 19*8(x31); ld x20, 20*8(x31); ld x21, 21*8(x31); ld x22, 22*8(x31); ld x23, 23*8(x31); ld x24, 24*8(x31); ld x25, 25*8(x31); ld x26, 26*8(x31); ld x27, 27*8(x31); ld x28, 28*8(x31); ld x29, 29*8(x31); ld x30, 30*8(x31); ld x31, 31*8(x31);
    sret
    