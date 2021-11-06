use core::mem::size_of;

use crate::arch::syscall;
use crate::arch::trap;

use super::register::TrapCauseLoaderImpl;
global_asm!(include_str!("trap.asm"));

#[repr(C)]
#[derive(Copy, Clone)] //这样就能正常拷贝了。Copy+Clone说明使用=的时候，不是使用move语义转移所有权，而是拷贝一份
pub struct TrapContextStoreImpl {
    //需要保存32寄存器，以及sepc, sscratch
    //x0..=x32, sepc, sscratch，一共34个，sscratch用于保存内核态指针
    ctx: [u64; 34], //使用数组，方便直接汇编的时候操作，免得有对齐的问题
}

impl TrapContextStoreImpl {
    pub fn save(&self) -> &'static TrapContextStoreImpl {
        unsafe {
            let ctx = (self.ctx[33] as *mut TrapContextStoreImpl)
                .as_mut()
                .unwrap();
            *ctx = *self;
            ctx
        }
    }
}

impl trap::TrapContextStore for TrapContextStoreImpl {
    fn set_sp(&mut self, sp: u64) {
        self.ctx[2] = sp; //x2就是sp
    }

    fn set_pc(&mut self, pc: u64) {
        self.ctx[32] = pc; //x32就是sepc
    }
    fn set_kernel_stack(&mut self, stack_top: u64) {
        self.ctx[33] = stack_top - size_of::<TrapContextStoreImpl>() as u64; //预先分配个空间
    }

    fn mv_pc_to_next(&mut self) {
        self.ctx[32] += 4; //仅用于syscall场景，触发syscall的ecall指令是4字节长度
    }

    fn restore_trap(&self) -> ! {
        //根据自己保存的上下文恢复到用户态
        extern "C" {
            fn restore_trap_asm(ctx: &TrapContextStoreImpl) -> !;
        }
        // kinfo!("restore");
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
        TrapContextStoreImpl { ctx: [0; 34] }
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
fn trap_entry(ctx: &'static TrapContextStoreImpl) -> &'static TrapContextStoreImpl {
    // kinfo!("trap entry");
    trap::dispatch_trap(&mut trap::TrapContext::new(ctx))
        .raw()
        .save()
}
