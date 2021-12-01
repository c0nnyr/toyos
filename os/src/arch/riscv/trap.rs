use crate::arch::riscv::register;
use crate::arch::syscall;
use crate::arch::trap;
use crate::arch::trap::TrapContextStore;
use crate::mm::KERNEL_PAGE_TABLE_TREE;
global_asm!(include_str!("trap.asm"));

#[repr(C)]
pub struct TrapContextStoreImpl {
    //需要保存32寄存器，以及sepc
    //x0..=x32, sepc，satp内核态, satp用户态
    ctx: [u64; 35], //使用数组，方便直接汇编的时候操作，免得有对齐的问题
}

impl trap::TrapContextStore for TrapContextStoreImpl {
    fn set_page_table_root_ppn(&mut self, root: u64, is_user: bool) {
        let satp = register::SAtp::from_ppn(root as usize);
        if is_user {
            self.ctx[34] = satp.bits as u64;
        } else {
            self.ctx[33] = satp.bits as u64;
        }
    }

    fn set_sp(&mut self, sp: u64) {
        self.ctx[2] = sp; //x2就是sp
    }

    fn set_pc(&mut self, pc: u64) {
        self.ctx[32] = pc; //x32就是sepc
    }

    fn mv_pc_to_next(&mut self) {
        self.ctx[32] += 4; //仅用于syscall场景，触发syscall的ecall指令是4字节长度
    }

    fn restore_trap(&self) -> ! {
        //根据自己保存的上下文恢复到用户态
        extern "C" {
            fn restore_trap_asm(ctx: &TrapContextStoreImpl) -> !;
        }
        unsafe { restore_trap_asm(self) }
    }

    fn get_syscall_param(&self) -> syscall::SyscallParam {
        syscall::SyscallParam {
            params: [
                //使用a0~a3作为参数
                self.ctx[10] as usize,
                self.ctx[11] as usize,
                self.ctx[12] as usize,
                self.ctx[13] as usize,
            ],
            syscall_id: syscall::SyscallId::from(self.ctx[17] as usize), //使用a7作为syscall id，传统而已，无所谓
        }
    }
    fn set_syscall_return_code(&mut self, code: usize) {
        self.ctx[10] = code as u64;
    }
}
impl Default for TrapContextStoreImpl {
    fn default() -> Self {
        TrapContextStoreImpl { ctx: [0; 35] }
    }
}

pub fn init() {
    extern "C" {
        fn init_trap_entry_asm();
    }
    unsafe {
        init_trap_entry_asm();
    }
}

#[no_mangle]
fn trap_entry(ctx: &'static mut trap::TrapContext) -> &'static trap::TrapContext {
    trap::dispatch_trap(ctx)
}
