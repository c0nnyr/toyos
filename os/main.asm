#main.asm
    .section .text
    .globl _start  #这个符号需要声明，链接器是需要导出到ELF中的
_start:
    la t0, message # 加载当前要打印的字符的地址，伪指令la，展开就是addi
    lb t1, 0(t0) # 加载当前要打印的字符，这是读内存，一个byte
putchar: #定义一个局部的符号，表示当前指令的地址（也就是下一条mv）
    mv a0, t1 # 设置打印字符的参数
    mv a1, zero #未使用
    mv a2, zero #未使用
    addi a7, zero, 1 # 打印字符的操作吗
    ecall 
    addi t0, t0, 1 #地址+1
    lb t1, 0(t0) # 当前要打印的字符
    bne t1, zero, putchar # 如果当前字符不是空字符串（结束符），则跳转到putchar位置
dead_loop:
    jal        dead_loop  #死循环，也是伪指令

    .section .rodata  #定义在readonlydata这个段，这是惯例，Hello world字符串就是常量
message: #message这个字符串的开始位置
    .string "Hello, world!"  #会自动添加一个\0的结束符
    
