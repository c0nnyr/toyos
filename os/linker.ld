# 语法有点类似汇编
KERNEL_MAIN_PHY_ADDR = 0x80200000; # 内核加载的地址，也是我们希望_start函数所处于的位置
SECTIONS
{
    . = KERNEL_MAIN_PHY_ADDR; # 表示这个地方地址是KERNEL_MAIN_PHY_ADDR
    .text : {  # 都归在.text段
        *(.text .text.*) # 表示所有.text和.text.*这些段，0个或者多个。 下面那些同理
    }
    # 为啥要对齐？因为我们是64位的，CPU在加载数据的时候都是8字节对齐，所以我们得要求数据段都是8字节对齐的。
    . = ALIGN(8); 
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    . = ALIGN(8);
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    . = ALIGN(8);
    .bss : {
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
}
