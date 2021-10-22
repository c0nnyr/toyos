use super::task;
use crate::arch::trap::{self, TrapContext, TrapContextStore};

pub const MAX_TASK_NUM: usize = 100; //最大支持的TASK数量
pub const MAX_TASK_SIZE: usize = 0x100000; //TASK包体的最大尺寸，1M
pub const TASK_RUNNING_ADDR: usize = 0x80400000; //TASK运行地址
static USER_STACK: [u8; 4096] = [0; 4096];

pub struct TaskManager {
    tasks: [Option<task::Task>; MAX_TASK_NUM],
    current_idx: usize, // 记录当前运行的TASK
}

impl TaskManager {
    pub fn init_tasks(&mut self) {
        let mut task_num: usize = 0;
        unsafe {
            task_num = *(TASK_RUNNING_ADDR as *const usize); //最开始的task数量放在这个位置，这个读取也是不安全的
            assert!(
                task_num <= MAX_TASK_NUM,
                "task_num must be less than {} but get {}",
                MAX_TASK_NUM,
                task_num
            );
        }
        for i in 0..task_num {
            //从 TASK_RUNNING_ADDR + MAX_TASK_SIZE开始是第一个应用，每个应用MAX_TASK_SIZE大小
            self.tasks[i] = Some(task::Task::new(
                TASK_RUNNING_ADDR + (i + 1) * MAX_TASK_SIZE,
                TASK_RUNNING_ADDR + (i + 2) * MAX_TASK_SIZE,
            ));
        }
    }

    //Rust里面的错误，本质上也是个泛型枚举。这里我们的()是数据的类型，静态生命周期字符串引用&'static str（也就是字符串常量）是错误的类型
    fn load_task_code(&self, idx: usize) -> Result<(), &'static str> {
        kinfo!("==============================\nLoadingTask {}", idx);
        if idx >= MAX_TASK_NUM {
            return Err("idx exceed max task num");
        }
        match &self.tasks[idx] {
            //避免拷贝，所以才加上个&
            Some(task) => {
                let code = task.get_code();
                unsafe {
                    let dst: &mut [u8] =//这里需要的是一个可变的数组引用，也同样是不安全的
                        core::slice::from_raw_parts_mut(TASK_RUNNING_ADDR as *mut u8, code.len());
                    dst.copy_from_slice(code); //拷贝代码
                }
                Ok(())
            }
            None => Err("no task to load"),
        }
    }

    pub fn switch_to_task(&mut self, idx: usize) -> Result<(), &'static str> {
        //这个“？”是Rust的语法糖，如果是Err就直接原样Return了。适用于透传Err的那些场景。
        //相当于match self.load_task_code(idx){Err(err)=>{return err;},Ok(v)=>{xx}}
        //这样写可以降低圈复杂度，减少代码量
        self.load_task_code(idx)?;
        self.current_idx = idx;
        Ok(())
    }

    pub fn get_default_trap_context(&self) -> TrapContext {
        let mut ctx = trap::TrapContext::default();
        ctx.set_sp(USER_STACK.as_ptr() as u64 + USER_STACK.len() as u64);
        ctx.set_pc(TASK_RUNNING_ADDR as u64);
        ctx
    }
}

pub static TASK_MANAGER: spin::Mutex<TaskManager> = spin::Mutex::new(TaskManager {
    tasks: [None; MAX_TASK_NUM],
    current_idx: 0,
});

pub fn init() {
    TASK_MANAGER.lock().init_tasks();
}
