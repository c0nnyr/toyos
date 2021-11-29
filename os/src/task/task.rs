use core::mem::{self, size_of};

use crate::{
    arch::switch,
    arch::trap::{self, TrapContext, TrapContextStore},
    mm::{
        self,
        addr::{self, PAGE_SIZE},
        page_table::{self, PageTableTree},
        ppn_manager::{self, PPNManager},
        raw_page,
        section::{self, DATA_PERMISSION},
    },
}; //引入TrapContextStore才能使用TrapContext身上对这个trait的实现

#[derive(Clone, Copy, PartialEq)]
pub enum TaskState {
    Init,
    Running,
    Stopped,
}

const KERNEL_STACK_PAGE_NUM: usize = 2;
pub struct KernelStackPage {
    vpn: Option<addr::VirtualPageNumber>,
    ppn: ppn_manager::PhysicalPageNumberGuard,
    raw: &'static mut [u8; addr::PAGE_SIZE],
}

impl From<ppn_manager::PhysicalPageNumberGuard> for KernelStackPage {
    fn from(v: ppn_manager::PhysicalPageNumberGuard) -> Self {
        let raw = unsafe {
            (v.ppn.as_addr() as *mut [u8; addr::PAGE_SIZE])
                .as_mut()
                .unwrap()
        };
        Self {
            ppn: v,
            raw,
            vpn: None,
        }
    }
}

impl KernelStackPage {
    fn bind_vpn(&mut self, vpn: addr::VirtualPageNumber) {
        self.vpn = Some(vpn);
    }
}

pub struct KernelStack {
    kernel_stack: [KernelStackPage; KERNEL_STACK_PAGE_NUM],
}

impl KernelStack {
    pub fn init() -> Result<Self, &'static str> {
        let ppn_manager_inner = &mut ppn_manager::PPN_MANAGER.lock();
        Ok(Self {
            kernel_stack: [
                ppn_manager_inner.alloc()?.into(),
                ppn_manager_inner.alloc()?.into(),
            ],
        })
    }

    pub fn bind_vpn(&mut self, vpn_end: addr::VirtualPageNumber) {
        for (idx, kernel_stack_page) in self.kernel_stack.iter_mut().rev().enumerate() {
            kernel_stack_page.bind_vpn(addr::VirtualPageNumber::from(vpn_end.bits - idx - 1));
        }
    }
    //KernelStackPage高地址存放trapcontext
    pub fn get_trap_context(&self) -> &'static trap::TrapContext {
        unsafe {
            ((self.kernel_stack.last().unwrap().vpn.unwrap().as_addr() + PAGE_SIZE
                - size_of::<switch::TaskContext>()
                - size_of::<trap::TrapContext>()) as *const trap::TrapContext)
                .as_ref()
                .unwrap()
        }
    }

    pub fn get_trap_context_mut(&mut self) -> &'static mut trap::TrapContext {
        unsafe {
            ((self.kernel_stack.last().unwrap().vpn.unwrap().as_addr() + PAGE_SIZE
                - size_of::<switch::TaskContext>()
                - size_of::<trap::TrapContext>()) as *mut trap::TrapContext)
                .as_mut()
                .unwrap()
        }
    }
    pub fn get_task_context(&self) -> &'static switch::TaskContext {
        unsafe {
            ((self.kernel_stack.last().unwrap().vpn.unwrap().as_addr() + PAGE_SIZE
                - size_of::<switch::TaskContext>()) as *const switch::TaskContext)
                .as_ref()
                .unwrap()
        }
    }

    pub fn get_task_context_mut(&mut self) -> &'static mut switch::TaskContext {
        unsafe {
            ((self.kernel_stack.last().unwrap().vpn.unwrap().as_addr() + PAGE_SIZE
                - size_of::<switch::TaskContext>()) as *mut switch::TaskContext)
                .as_mut()
                .unwrap()
        }
    }
    pub fn map_in_kernel(&self) -> Result<(), &'static str> {
        let kernel_page_table = &mut mm::KERNEL_PAGE_TABLE_TREE.lock();
        for kernel_stack in self.kernel_stack.iter() {
            let section = section::VirtualSection::new(
                kernel_stack.vpn.unwrap().as_addr(),
                kernel_stack.vpn.unwrap().as_addr() + addr::PAGE_SIZE,
                section::MapTarget::BiasPageNumber(
                    kernel_stack.ppn.ppn.bits as i32 - kernel_stack.vpn.unwrap().bits as i32,
                ),
                section::DATA_PERMISSION.for_kernel(),
            );
            for (vpn, entry, _) in section.iter() {
                kernel_page_table.map(vpn, entry)?;
            }
        }
        Ok(())
    }
}

pub struct Task {
    pub start_addr: usize,
    end_addr: usize,
    state: TaskState,
    raw_pages: [Option<(
        addr::VirtualPageNumber,
        page_table::PageTableEntry,
        raw_page::RawPage,
    )>; 100], //暂时先用20个来撑一下
    page_table_tree: page_table::PageTableTree,
    pub kernel_stack: KernelStack,
}

fn task_first_entry(ctx: &'static TrapContext) {
    kinfo!("in task_first_entry");
    ctx.restore_trap()
}

impl Task {
    pub fn new(idx: usize, start_addr: usize, end_addr: usize) -> Self {
        let kernel_stack = KernelStack::init().unwrap();

        // let section = section::VirtualSection::new(addr::TOPEST_ADDR - addr::PAGE_SIZE*);
        // for (vpn, entry, _) in section.iter() {
        //     self.page_table_tree.map(vpn, entry);
        // }

        let mut task = Task {
            start_addr,
            end_addr,
            // trap_context: trap::TrapContext::default(),
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
        task.kernel_stack.bind_vpn(addr::VirtualPageNumber::ceil(
            Task::get_kernel_stack_top_virtual_addr(idx, task.kernel_stack.kernel_stack.len()),
        ));
        task.kernel_stack.map_in_kernel().unwrap();
        task.kernel_stack
            .get_trap_context_mut()
            .set_trap_entry(trap::trap_entry as u64);
        task.page_table_tree.init().unwrap();
        task.kernel_stack
            .get_trap_context_mut()
            .set_page_table_root_ppn(
                task.get_page_table_root_ppn().bits as u64,
                mm::KERNEL_PAGE_TABLE_TREE.lock().get_root_ppn().bits as u64,
            );
        let task_ctx = task.kernel_stack.get_task_context_mut();
        task_ctx.set_idx(idx);
        task_ctx.set_ra(task_first_entry as usize);
        task_ctx.set_sp(Task::get_kernel_stack_top_virtual_addr(
            idx,
            task.kernel_stack.kernel_stack.len(),
        ));
        task_ctx.set_param0(task.kernel_stack.get_trap_context() as *const TrapContext as usize);
        task
    }

    // pub

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
        //需要确保映射好了在调用，也就是在taks.map后调用
        self.kernel_stack.get_trap_context()
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
        }
        Ok(())
    }

    pub fn get_kernel_stack_top_virtual_addr(idx: usize, kernel_pages: usize) -> usize {
        return addr::TOPEST_ADDR + 1
            - idx * kernel_pages * PAGE_SIZE
            - PAGE_SIZE
            - mem::size_of::<switch::TaskContext>()
            - mem::size_of::<trap::TrapContext>();
    }

    pub fn map(&mut self) -> Result<(), &'static str> {
        for item in self.raw_pages.iter().as_ref() {
            if let Some(item) = item {
                self.page_table_tree.map(item.0, item.1)?;
            }
        }
        extern "C" {
            fn kernel_text_trap_start_asm();
            fn kernel_text_trap_end_asm();
        }
        //映射trap
        let section_def = [(
            addr::TOPEST_ADDR - PAGE_SIZE + 1,
            addr::TOPEST_ADDR + 1,
            section::MapTarget::BiasPageNumber(
                addr::PhysicalPageNumber::floor(kernel_text_trap_start_asm as usize).bits as i32
                    - addr::VirtualPageNumber::floor(addr::TOPEST_ADDR - PAGE_SIZE + 1).bits as i32,
            ),
            section::TEXT_PERMISSION.for_kernel(),
        )];

        for item in section_def {
            let section = section::VirtualSection::new(item.0, item.1, item.2, item.3);
            for (vpn, entry, _) in section.iter() {
                self.page_table_tree.map(vpn, entry);
            }
        }

        for (idx, kernel_stack) in self.kernel_stack.kernel_stack.iter().enumerate() {
            let section = section::VirtualSection::new(
                kernel_stack.vpn.unwrap().as_addr(),
                kernel_stack.vpn.unwrap().as_addr() + addr::PAGE_SIZE,
                section::MapTarget::BiasPageNumber(
                    kernel_stack.ppn.ppn.bits as i32 - kernel_stack.vpn.unwrap().bits as i32,
                ),
                section::DATA_PERMISSION.for_kernel(),
            );
            for (vpn, entry, _) in section.iter() {
                self.page_table_tree.map(vpn, entry);
            }
        }
        Ok(())
    }

    pub fn get_page_table_root_ppn(&self) -> addr::PhysicalPageNumber {
        self.page_table_tree.get_root_ppn()
    }
}
