use super::config;
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
            match v.get_code() {
                0x05 => trap::TrapCause::Interrupt(trap::Interrupt::Timer),
                _ => trap::TrapCause::Interrupt(trap::Interrupt::Unsupported(v.get_code())),
            }
        } else {
            match v.get_code() {
                8 => trap::TrapCause::Exeption(trap::Exception::Syscall),
                _ => trap::TrapCause::Exeption(trap::Exception::Unsupported(v.get_code())),
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Time {
    pub bits: usize,
}

impl Time {
    pub fn load() -> Self {
        extern "C" {
            fn load_time_asm() -> usize;
        }
        unsafe {
            Time {
                bits: load_time_asm(),
            }
        }
    }

    pub fn as_duration(&self) -> core::time::Duration {
        core::time::Duration::from_millis((self.bits / config::CLOCKS_PER_MS) as u64)
    }
}

#[derive(Copy, Clone)]
pub struct SIe {
    pub bits: usize,
}

impl SIe {
    fn set_bit(pos: usize) {
        assert!(pos >= 0 && pos < 64);
        extern "C" {
            fn set_sie_bit_asm(pos: usize) -> usize;
        }
        unsafe {
            set_sie_bit_asm(1 << pos);
        }
    }

    pub fn enable_time_interrupt() {
        Self::set_bit(0x05); //STIE
    }
}

#[derive(Copy, Clone)]
pub struct SAtp {
    pub bits: usize,
}

impl SAtp {
    pub fn set(&self) {
        extern "C" {
            fn set_satp_asm(satp: usize) -> usize;
        }
        unsafe {
            set_satp_asm(self.bits);
        }
    }

    pub fn from_ppn(ppn: usize) -> Self {
        let mode = 8 << 60; //60~63位为mode，8表示使用Sv39模式，虚拟地址只使用低39位，高25位必须为0
        Self{bits:mode | ppn}
    }
}
