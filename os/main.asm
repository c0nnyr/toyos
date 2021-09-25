#main.asm
addi a0, zero, 72 # 设置打印字符的参数: H
addi a1, zero, 0 
addi a2, zero, 0 
addi a7, zero, 1 # 打印字符的操作吗
ecall #发起调用，陷入机器模式（M模式），调用BIOS提供的打印中断
addi a0, zero, 101 # 设置打印字符的参数: e
addi a1, zero, 0 
addi a2, zero, 0 
addi a7, zero, 1 
ecall 
addi a0, zero, 108 # 设置打印字符的参数: l
addi a1, zero, 0 
addi a2, zero, 0 
addi a7, zero, 1 
ecall 
addi a0, zero, 108 # 设置打印字符的参数: l
addi a1, zero, 0 
addi a2, zero, 0 
addi a7, zero, 1 
ecall 
addi a0, zero, 111 # 设置打印字符的参数: o
addi a1, zero, 0 
addi a2, zero, 0 
addi a7, zero, 1 
ecall 
addi a0, zero, 44 # 设置打印字符的参数: ,
addi a1, zero, 0 
addi a2, zero, 0 
addi a7, zero, 1 
ecall 
addi a0, zero, 119 # 设置打印字符的参数: w
addi a1, zero, 0 
addi a2, zero, 0 
addi a7, zero, 1 
ecall 
addi a0, zero, 111 # 设置打印字符的参数: o
addi a1, zero, 0 
addi a2, zero, 0 
addi a7, zero, 1 
ecall 
addi a0, zero, 114 # 设置打印字符的参数: r
addi a1, zero, 0 
addi a2, zero, 0 
addi a7, zero, 1 
ecall 
addi a0, zero, 108 # 设置打印字符的参数: l
addi a1, zero, 0 
addi a2, zero, 0 
addi a7, zero, 1 
ecall 
addi a0, zero, 100 # 设置打印字符的参数: d
addi a1, zero, 0 
addi a2, zero, 0 
addi a7, zero, 1 
ecall 
addi a0, zero, 33 # 设置打印字符的参数: !
addi a1, zero, 0 
addi a2, zero, 0 
addi a7, zero, 1 
ecall 
auipc a0, 0  #保存当前指令的PC到a0。auipc就是add immediate to pc
jalr  ra, a0, 0 #死循环
