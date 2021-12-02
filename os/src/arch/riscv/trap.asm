    .section .text.trap
    .balign 8
enter_trap_asm:
    csrrw x31, sscratch, x31 # 暂时交换保存一下x31
    sd x1, 1*8(x31); sd x2, 2*8(x31); sd x3, 3*8(x31); sd x4, 4*8(x31); sd x5, 5*8(x31); sd x6, 6*8(x31); sd x7, 7*8(x31); sd x8, 8*8(x31); sd x9, 9*8(x31); sd x10, 10*8(x31); sd x11, 11*8(x31); sd x12, 12*8(x31); sd x13, 13*8(x31); sd x14, 14*8(x31); sd x15, 15*8(x31); sd x16, 16*8(x31); sd x17, 17*8(x31); sd x18, 18*8(x31); sd x19, 19*8(x31); sd x20, 20*8(x31); sd x21, 21*8(x31); sd x22, 22*8(x31); sd x23, 23*8(x31); sd x24, 24*8(x31); sd x25, 25*8(x31); sd x26, 26*8(x31); sd x27, 27*8(x31); sd x28, 28*8(x31); sd x29, 29*8(x31); sd x30, 30*8(x31)
    # 可以正常使用x30了，把真正的x31保存进去
    csrr x30, sscratch #t5=x30
    sd x30, 31*8(x31) 
    # 保存sepc
    csrr x30, sepc 
    sd x30, 32*8(x31) 

    ld x30, 33*8(x31)  # 读取内核satp
    csrw satp, x30
    sfence.vma

    mv x2, x31
    # fn (&TrapContext)->&TrapContext
    mv x10, x31 # 设置参数用于消费上下文

    ld x30, 35*8(x31)  #trap_entry的虚拟地址
    jalr x30
restore_trap_asm:
    # fn (ptr *TrapContextStore)
    # a0=x10=ptr
    mv x31, x10 # t6=x31
    csrw sscratch, x31 # 存放内核态的栈
    ld x30, 32*8(x31) #t5=x30
    csrw sepc, x30

    ld x30, 34*8(x31)  # 读取用户态satp
    csrw satp, x30
    sfence.vma

    ld x1, 1*8(x31); ld x2, 2*8(x31); ld x3, 3*8(x31); ld x4, 4*8(x31); ld x5, 5*8(x31); ld x6, 6*8(x31); ld x7, 7*8(x31); ld x8, 8*8(x31); ld x9, 9*8(x31); ld x10, 10*8(x31); ld x11, 11*8(x31); ld x12, 12*8(x31); ld x13, 13*8(x31); ld x14, 14*8(x31); ld x15, 15*8(x31); ld x16, 16*8(x31); ld x17, 17*8(x31); ld x18, 18*8(x31); ld x19, 19*8(x31); ld x20, 20*8(x31); ld x21, 21*8(x31); ld x22, 22*8(x31); ld x23, 23*8(x31); ld x24, 24*8(x31); ld x25, 25*8(x31); ld x26, 26*8(x31); ld x27, 27*8(x31); ld x28, 28*8(x31); ld x29, 29*8(x31); ld x30, 30*8(x31); ld x31, 31*8(x31);
    sret
init_trap_entry_asm:
    csrw stvec, a0
    ret
call_virtual_restore_trap_asm:
    jalr a1 #返回地址放在了ra中, a0是trapcontext
    ret
    