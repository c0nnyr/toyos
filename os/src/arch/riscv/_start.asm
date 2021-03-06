#main.asm
    .section .text._start
    .globl _start # 必须要，符号要导出去
_start:
    la sp, boot_kernal_stack_end_asm # 赋值，栈是从高地址到低地址的
    call main # 伪指令，等价于auipc计算一下跳转的地方，jalr跳转一下

    .section .bss.boot_stack #为啥要在bss段？传统，这种初始化全0的定义在bss段，ELF中就可以只记录变量的大小，不用分配真正空间，达到减小可执行程序、提高运行加载效率的目的
    .globl boot_kernal_stack_end_asm
    .balign 8 #还是按照8字节对齐。也可以用.align 3，表示按照2^3对齐
boot_kernal_stack_start_asm:
    .space 40960 #定义40960，也就是40K字节
boot_kernal_stack_end_asm: 