use core::mem;

use crate::{
    arch::trap::{self, TrapContextStore},
    mm::{
        self,
        addr::{self, PAGE_SIZE},
        page_table::{self, PageTableTree},
        ppn_manager::{PPNManager, PPN_MANAGER},
        raw_page,
        section::{self, DATA_PERMISSION},
    },
    task::{kernel_stack, task_manager::TASK_MANAGER},
}; //引入TrapContextStore才能使用TrapContext身上对这个trait的实现

#[derive(Clone, Copy, PartialEq)]
pub enum TaskState {
    Init,
    Running,
    Stopped,
}

pub struct Task {
    pub start_addr: usize,
    idx: usize,
    end_addr: usize,
    state: TaskState,
    raw_pages: [Option<(
        addr::VirtualPageNumber,
        page_table::PageTableEntry,
        raw_page::RawPage,
    )>; 100], //暂时先用20个来撑一下
    page_table_tree: page_table::PageTableTree,
    pub kernel_stack: kernel_stack::KernelStack,
}

fn first_restore_trap() {
    let ctx = || {
        let mut mgr = TASK_MANAGER.lock();
        let cur_idx = kernel_stack::KernelStack::get_idx();
        let task = mgr.get_task_mut(cur_idx).unwrap();
        task.load_code().unwrap();
        let ctx = task.kernel_stack.get_trap_context();
        ctx
    };
    ctx().restore_trap()
}

impl Task {
    pub fn new(idx: usize, start_addr: usize, end_addr: usize) -> Self {
        let mut kernel_stack: kernel_stack::KernelStack =
            PPN_MANAGER.lock().alloc().unwrap().into();
        kernel_stack.task_idx = Some(idx);
        let mut task = Task {
            idx,
            start_addr,
            end_addr,
            state: TaskState::Init,
            raw_pages: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None,
            ],
            page_table_tree: page_table::PageTableTree::default(),
            kernel_stack,
        };
        task.page_table_tree.init().unwrap();
        task.map_for_kernel().unwrap(); //映射trap相关的虚拟地址，然后就可以正常使用trap_context相关的虚拟地址了
        let trap_context = task.kernel_stack.get_trap_context_mut();
        *trap_context = trap::TrapContext::default();
        trap_context.set_page_table_root_ppn(task.page_table_tree.get_root_ppn().bits as u64, true);
        trap_context.set_page_table_root_ppn(
            mm::KERNEL_PAGE_TABLE_TREE.lock().get_root_ppn().bits as u64,
            false,
        );
        trap_context.set_trap_handler(trap::trap_entry as u64);
        let task_ctx = trap_context.get_task_context_mut();
        task_ctx.set_return_addr(first_restore_trap as usize);
        task_ctx.set_sp(task.kernel_stack.get_top());
        task
    }

    pub fn get_code(&self) -> &[u8] {
        unsafe {
            // 从直接裸的指针返回slice，不安全，因为这块内存谁是owner，谁会改动，rust不知道，只有我们写程序的作者知道。
            // 我们是确信的，这块内存是只读的，没人改，因而返回一个只读的slice引用是安全的。
            core::slice::from_raw_parts(
                self.start_addr as *const u8,
                self.end_addr - self.start_addr,
            )
        }
    }

    pub fn get_trap_context(&self) -> &'static trap::TrapContext {
        self.kernel_stack.get_trap_context()
    }

    pub fn get_trap_context_mut(&mut self) -> &'static mut trap::TrapContext {
        self.kernel_stack.get_trap_context_mut()
    }

    pub fn set_state(&mut self, state: TaskState) {
        self.state = state;
    }

    pub fn is_runnable(&self) -> bool {
        self.state == TaskState::Init || self.state == TaskState::Running
    }

    fn save_raw_page(
        &mut self,
        vpn: addr::VirtualPageNumber,
        entry: page_table::PageTableEntry,
        raw_page: raw_page::RawPage,
    ) -> Result<(), &'static str> {
        for (i, item) in self.raw_pages.iter().enumerate() {
            if item.is_none() {
                self.raw_pages[i] = Some((vpn, entry, raw_page));
                return Ok(());
            }
        }
        Err("failed to save raw page")
    }

    pub fn load_code(&mut self) -> Result<(), &'static str> {
        if self.raw_pages[0].is_none() {
            let raw_data = unsafe {
                core::slice::from_raw_parts(
                    self.start_addr as *const u8,
                    self.end_addr - self.start_addr,
                )
            };
            let elf = xmas_elf::ElfFile::new(raw_data)?;
            let elf_header = elf.header;
            let ph_count = elf_header.pt2.ph_count();
            let mut max_end_vpn = addr::VirtualPageNumber::from(0); //之后用于计算代码的上限，选择虚拟地址中合适的栈开始位置
            for i in 0..ph_count {
                let ph = elf.program_header(i).unwrap();
                if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                    let permission = crate::mm::section::Permission {
                        user: true,
                        read: ph.flags().is_read(),
                        write: ph.flags().is_write(),
                        execute: ph.flags().is_execute(),
                    };
                    let data: &[u8] =
                        &elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize];
                    let section = section::VirtualSection::new(
                        ph.virtual_addr() as usize,
                        (ph.virtual_addr() + ph.mem_size()) as usize,
                        section::MapTarget::Random(Some(data)),
                        permission,
                    );
                    for (vpn, entry, raw_page) in section.iter() {
                        self.save_raw_page(vpn, entry, raw_page.unwrap())?;
                    }
                    if section.end_vpn.bits > max_end_vpn.bits {
                        max_end_vpn = section.end_vpn;
                    }
                }
            }
            let stack_vpn = addr::VirtualPageNumber::from(max_end_vpn.bits + 1); //留出4K的保护区间
            let section = section::VirtualSection {
                start_vpn: stack_vpn,
                end_vpn: addr::VirtualPageNumber::from(stack_vpn.bits + 1), //4K
                map_target: section::MapTarget::Random(None),
                permission: DATA_PERMISSION.for_user(),
            };
            for (vpn, entry, raw_page) in section.iter() {
                self.save_raw_page(vpn, entry, raw_page.unwrap())?;
            }
            let entry_addr = elf_header.pt2.entry_point();

            self.kernel_stack.get_trap_context_mut().set_pc(entry_addr);
            self.kernel_stack
                .get_trap_context_mut()
                .set_sp(section.end_vpn.as_addr() as u64);
            self.map_for_user();
        }
        Ok(())
    }

    pub fn map_for_user(&mut self) -> Result<(), &'static str> {
        for item in self.raw_pages.iter().as_ref() {
            if let Some(item) = item {
                self.page_table_tree.map(item.0, item.1)?;
            }
        }
        return Ok(());
    }
    pub fn map_for_kernel(&mut self) -> Result<(), &'static str> {
        extern "C" {
            fn kernel_text_trap_start_asm();
            fn kernel_text_trap_end_asm();
        }
        //映射trap
        let section_def = [
            (
                addr::TRAP_ADDR,
                addr::TRAP_ADDR + addr::PAGE_SIZE,
                section::MapTarget::AlignTo(addr::PhysicalPageNumber::floor(
                    kernel_text_trap_start_asm as usize,
                )),
                section::TEXT_PERMISSION.for_kernel(),
            ),
            (
                self.kernel_stack.get_bottom(),
                self.kernel_stack.get_top(),
                section::MapTarget::AlignTo(self.kernel_stack.ppn.ppn),
                section::BSS_PERMISSION.for_kernel(),
            ),
        ];

        for item in section_def {
            let section = section::VirtualSection::new(item.0, item.1, item.2, item.3);
            for (vpn, entry, _) in section.iter() {
                self.page_table_tree.map(vpn, entry);
                mm::KERNEL_PAGE_TABLE_TREE.lock().map(vpn, entry);
            }
        }
        Ok(())
    }
}
