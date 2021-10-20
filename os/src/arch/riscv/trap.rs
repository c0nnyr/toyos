use crate::arch::trap;
global_asm!(include_str!("trap.asm"));

pub struct TrapContextStoreImpl {
    //需要保存32寄存器，以及sepc
    //x0..=x32, sepc，一共33个
    ctx: [u64; 33], //使用数组，方便直接汇编的时候操作，免得有对齐的问题
}

impl trap::TrapContextStore for TrapContextStoreImpl {
    fn set_sp(&mut self, sp: u64) {
        self.ctx[2] = sp; //x2就是sp
    }

    fn set_pc(&mut self, pc: u64) {
        self.ctx[32] = pc; //x32就是sepc
    }

    fn restore_trap(&self) -> ! {
        //根据自己保存的上下文恢复到用户态
        extern "C" {
            fn restore_trap_asm(ctx: &TrapContextStoreImpl) -> !;
        }
        unsafe { restore_trap_asm(self) }
    }
}
impl Default for TrapContextStoreImpl {
    fn default() -> Self {
        TrapContextStoreImpl { ctx: [0; 33] }
    }
}
