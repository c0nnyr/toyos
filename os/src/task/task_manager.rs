use super::task;
use crate::arch::trap::TrapContextStore;
use crate::arch::{time, trap};
use crate::mm::{self, addr, page_table};

pub const MAX_TASK_NUM: usize = 100; //最大支持的TASK数量
pub const MAX_TASK_SIZE: usize = 0x100000; //TASK包体的最大尺寸，1M
pub const TASK_RUNNING_ADDR: usize = 0x80400000; //TASK运行地址
pub const USER_STACK_SIZE: usize = 4096;

pub struct TaskManager {
    tasks: [Option<task::Task>; MAX_TASK_NUM],
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
                i,
                TASK_RUNNING_ADDR + (i + 1) * MAX_TASK_SIZE,
                TASK_RUNNING_ADDR + (i + 2) * MAX_TASK_SIZE,
            ));
        }
    }

    pub fn switch_to_next_task(&mut self, from_idx: usize) -> Result<usize, &'static str> {
        for i in 0..MAX_TASK_NUM {
            let idx = (i + from_idx) % MAX_TASK_NUM; //循环一圈
            match &mut self.tasks[idx] {
                Some(task) => {
                    if task.is_runnable() {
                        kinfo!(
                            "==============================LoadingTask {} at {:?}.",
                            idx,
                            time::get_now()
                        );
                        task.load_code()?;
                        task.set_state(task::TaskState::Running);
                        return Ok(idx);
                    }
                }
                None => (),
            }
        }
        Err("no more task")
    }

    pub fn get_task_mut(&mut self, idx: usize) -> Option<&mut task::Task> {
        (&mut self.tasks[idx]).as_mut()
    }
    pub fn get_task(&self, idx: usize) -> Option<&task::Task> {
        (&self.tasks[idx]).as_ref()
    }
}

pub static TASK_MANAGER: spin::Mutex<TaskManager> = spin::Mutex::new(TaskManager {
    tasks: [
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None,
    ],
});

pub fn init() {
    TASK_MANAGER.lock().init_tasks();
}
