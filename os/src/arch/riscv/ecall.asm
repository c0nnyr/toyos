    .section .text
ecall_asm: # 用于调用ecall，直接把函数传递4个参数的按照ecall的要求传递a0->a0, a1->a1, a2->a2, a3->a7
    #函数 fn ecall_asm(a0,a1,a2,a3)->a0
    mv a7, a3 
    ecall
    ret