load_scause_asm:
    csrr a0, scause
    ret
load_time_asm:
    csrr a0, time
    ret
set_sie_bit_asm:
    csrrs a0, sie, a0
    ret
set_satp_asm:
    csrrw a0, satp, a0
    sfence.vma #刷新TLB
    ret