#main.asm
    .section .text._start
    .globl _start # 必须要，符号要导出去
_start:
    la sp, boot_kernal_stack_end_asm # 赋值，栈是从高地址到低地址的
    call main # 伪指令，等价于auipc计算一下跳转的地方，jalr跳转一下
    la t0, trap_entry_asm
    csrw stvec, t0
    .balign 8
trap_entry_asm:
    call trap_entry

    .section .bss #为啥要在bss段？传统，这种初始化全0的定义在bss段，ELF中就可以只记录变量的大小，不用分配真正空间，达到减小可执行程序、提高运行加载效率的目的
    .balign 8 #还是按照8字节对齐。也可以用.align 3，表示按照2^3对齐
boot_kernal_stack_start_asm:
    .space 4096 #定义4096，也就是4K字节
boot_kernal_stack_end_asm: 