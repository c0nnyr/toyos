use crate::arch::trap;
global_asm!(include_str!("register.asm"));

pub struct SCause {
    bits: u64,
}

impl SCause {
    fn load() -> Self {
        extern "C" {
            fn load_scause_asm() -> u64;
        }
        unsafe {
            SCause {
                bits: load_scause_asm(),
            }
        }
    }

    fn is_interrupt(&self) -> bool {
        //最高位表示是interrupt还是exception
        self.bits & (1 << 63) != 0
    }

    fn get_code(&self) -> usize {
        //提取除了最高位的信息作为code
        (self.bits as usize) & !(1 << 63)
    }
}

pub struct TrapCauseLoaderImpl {}

impl trap::TrapCauseLoader for TrapCauseLoaderImpl {
    fn load() -> trap::TrapCause {
        let v = SCause::load();
        if v.is_interrupt() {
            trap::TrapCause::Interrupt(trap::Interrupt::Unsupported(v.get_code()))
        } else {
            trap::TrapCause::Exeption(trap::Exception::Unsupported(v.get_code()))
        }
    }
}
