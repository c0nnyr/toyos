load_scause_asm:
    csrr a0, scause
    ret
load_time_asm:
    csrr a0, time
    ret
set_sie_bit_asm:
    csrrs a0, sie, a0 #设置某个bit，a0二进制展开后bit为1对应的那些位设置唯一。比如设置N位，则应该a0=1<<N
    ret
set_satp_asm:
    csrrw a0, satp, a0
    sfence.vma #刷新MMU
    ret