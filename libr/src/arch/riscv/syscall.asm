    .section .text
syscall_asm: # fn (a0, a1, a2, a3, syscall_id)->usize
    mv a7, a4 # a4是syscallId，mv到a7，继承传统
    ecall
    ret