load_scause_asm:
    csrr a0, scause
    ret
load_time_asm:
    csrr a0, time
    ret