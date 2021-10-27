load_scause_asm:
    csrr a0, scause
    ret
load_time_asm:
    csrr a0, time
    ret
set_sie_bit_asm:
    csrrs a0, sie, a0
    ret