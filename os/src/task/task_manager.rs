use super::task;
use crate::arch::{time, trap};
use crate::mm::{self, addr, page_table};

pub const MAX_TASK_NUM: usize = 100; //最大支持的TASK数量
pub const MAX_TASK_SIZE: usize = 0x100000; //TASK包体的最大尺寸，1M
pub const TASK_RUNNING_ADDR: usize = 0x80400000; //TASK运行地址
pub const USER_STACK_SIZE: usize = 4096;

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
        if idx >= MAX_TASK_NUM {
            return Err("idx exceed max task num");
        }
        match &self.tasks[idx] {
            //避免拷贝，所以才加上个&
            Some(task) => {
                kinfo!(
                    "==============================LoadingTask {} at {:?}.",
                    idx,
                    time::get_now()
                );

                let kernel_page_table_tree = &mut mm::KERNEL_PAGE_TABLE_TREE.lock();
                for i in 0..addr::VirtualPageNumber::floor(MAX_TASK_SIZE).bits {
                    let addr_offset = addr::VirtualPageNumber::from(i).as_addr();
                    //遍历task对应的页
                    let vpn = addr::VirtualPageNumber::floor(TASK_RUNNING_ADDR + addr_offset);
                    let ppn = addr::PhysicalPageNumber::floor(task.start_addr + addr_offset);
                    let entry = page_table::PageTableEntry {
                        ppn: ppn,
                        valid: true,
                        read: true,
                        write: true,
                        execute: true,
                        user: true,
                    };
                    kernel_page_table_tree.map(vpn, entry).unwrap();
                }
                kernel_page_table_tree.active(); //真正启用地址映射
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
        self.set_current_state(task::TaskState::Running);
        Ok(())
    }

    pub fn switch_to_next_task(&mut self, from_idx: usize) -> Result<(), &'static str> {
        for i in 0..MAX_TASK_NUM {
            let idx = (i + from_idx) % MAX_TASK_NUM; //循环一圈
            match &self.tasks[idx] {
                Some(task) => {
                    if task.is_runnable() {
                        return self.switch_to_task(idx);
                    }
                }
                None => (),
            }
        }
        Err("no more task")
    }

    pub fn get_current_idx(&self) -> usize {
        self.current_idx
    }

    pub fn get_current_trap_context(&self) -> trap::TrapContext {
        match &self.tasks[self.current_idx] {
            Some(task) => task.get_trap_context(),
            None => panic!("never here"),
        }
    }

    pub fn save_current_trap_context(&mut self, ctx: &trap::TrapContext) {
        match &mut self.tasks[self.current_idx] {
            Some(task) => task.save_trap_context(ctx),
            None => panic!("never here"),
        }
    }

    pub fn set_current_state(&mut self, state: task::TaskState) {
        match &mut self.tasks[self.current_idx] {
            Some(task) => task.set_state(state),
            None => panic!("never here"),
        }
    }
}

pub static TASK_MANAGER: spin::Mutex<TaskManager> = spin::Mutex::new(TaskManager {
    tasks: [None; MAX_TASK_NUM],
    current_idx: 0,
});

pub fn init() {
    TASK_MANAGER.lock().init_tasks();
}
