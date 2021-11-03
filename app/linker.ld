APP_MAIN_PHY_ADDR = 0; # 应用程序加载的地址
SECTIONS
{
    . = APP_MAIN_PHY_ADDR; # 表示这个地方地址是APP_MAIN_PHY_ADDR
    .text : {  
        *(.text._start) #_start需要在最前面，这样就能正确从_start启动了
        *(.text .text.*) 
    }
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
