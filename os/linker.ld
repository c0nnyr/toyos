# 语法有点类似汇编
KERNEL_MAIN_PHY_ADDR = 0x80200000; # 内核加载的地址，也是我们希望_start函数所处于的位置
SECTIONS
{
    . = KERNEL_MAIN_PHY_ADDR; # 表示这个地方地址是KERNEL_MAIN_PHY_ADDR
    kernel_text_start_asm = .;
    .text : {  # 都归在.text段
        *(.text._start) #_start需要在最前面，这样就能正确从_start启动了
        . = ALIGN(4K); 
        kernel_text_trap_start_asm = .;
        *(.text.trap) 
        kernel_text_trap_end_asm = .;
        . = ALIGN(4K); 
        *(.text .text.*) # 表示所有.text和.text.*这些段，0个或者多个。 下面那些同理
    }
    kernel_text_end_asm = .;
    . = ALIGN(4K); 
    kernel_rodata_start_asm = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    kernel_rodata_end_asm = .;
    . = ALIGN(4K);
    kernel_data_start_asm = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    kernel_data_end_asm = .;
    . = ALIGN(4K);

    # 留出 4K的空间，用于做GAP。将来不会对这块区域做地址映射
    kernel_bss_start_asm = . + 4K;
    . = kernel_bss_start_asm;
    .bss : {
        *(.bss.boot_stack) # 将栈放在最前面
        . = ALIGN(4K); 
        kernel_bss_trap_start_asm = .;
        *(.bss.trap) 
        kernel_bss_trap_end_asm = .;
        . = ALIGN(4K); 
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
    kernel_bss_end_asm = .;
    . = ALIGN(4K);
    kernel_end_asm = .;
}
