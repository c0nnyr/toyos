SECTIONS
{
    . = ALIGN(4K); 
    .text : {  
        *(.text._start) #_start需要在最前面，这样就能正确从_start启动了
        *(.text .text.*) 
    }
    . = ALIGN(4K); 
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    . = ALIGN(4K);
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    . = ALIGN(4K);
    .bss : {
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
}