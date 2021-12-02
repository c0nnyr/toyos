pub mod addr;
pub mod page_table;
pub mod ppn_manager;
pub mod raw_page;
pub mod section;
use crate::task::task_manager;

pub fn init() {
    ppn_manager::init();
    init_kernel_map();
}

//总需要个地方Own这个映射，否则申请的内存页都被释放了
//暂时放在这里
pub static KERNEL_PAGE_TABLE_TREE: spin::Mutex<page_table::PageTableTree> =
    spin::Mutex::new(page_table::PageTableTree::default());

fn init_kernel_map() {
    extern "C" {
        fn kernel_text_start_asm();
        fn kernel_text_end_asm();
        fn kernel_rodata_start_asm();
        fn kernel_rodata_end_asm();
        fn kernel_data_start_asm();
        fn kernel_data_end_asm();
        fn kernel_bss_start_asm();
        fn kernel_bss_end_asm();
        fn kernel_end_asm();

        fn kernel_text_trap_start_asm();
        fn kernel_text_trap_end_asm();
    }
    kinfo!("kernel_text_start: 0x{:x}", kernel_text_start_asm as usize);
    kinfo!("kernel_text_end: 0x{:x}", kernel_text_end_asm as usize);
    kinfo!(
        "kernel_rodata_start: 0x{:x}",
        kernel_rodata_start_asm as usize
    );
    kinfo!("kernel_rodata_end: 0x{:x}", kernel_rodata_end_asm as usize);
    kinfo!("kernel_data_start: 0x{:x}", kernel_data_start_asm as usize);
    kinfo!("kernel_data_end: 0x{:x}", kernel_data_end_asm as usize);
    kinfo!("kernel_bss_start: 0x{:x}", kernel_bss_start_asm as usize);
    kinfo!("kernel_bss_end: 0x{:x}", kernel_bss_end_asm as usize);
    kinfo!("kernel_end: 0x{:x}", kernel_end_asm as usize);

    let kernel_page_table_tree = &mut KERNEL_PAGE_TABLE_TREE.lock();
    kernel_page_table_tree.init().unwrap();

    let section_def = [
        (
            kernel_text_start_asm as usize,
            kernel_text_end_asm as usize,
            section::MapTarget::Identity,
            section::TEXT_PERMISSION.for_kernel(),
        ),
        (
            kernel_rodata_start_asm as usize,
            kernel_rodata_end_asm as usize,
            section::MapTarget::Identity,
            section::RODATA_PERMISSION.for_kernel(),
        ),
        (
            kernel_data_start_asm as usize,
            kernel_data_end_asm as usize,
            section::MapTarget::Identity,
            section::DATA_PERMISSION.for_kernel(),
        ),
        (
            kernel_bss_start_asm as usize,
            kernel_bss_end_asm as usize,
            section::MapTarget::Identity,
            section::BSS_PERMISSION.for_kernel(),
        ),
        (
            //这块内存用于动态分配，同时应用程序的数量也放在TASK_RUNNING_ADDR的位置呢
            kernel_end_asm as usize,
            (task_manager::TASK_RUNNING_ADDR + 10 * task_manager::MAX_TASK_SIZE) as usize,
            section::MapTarget::Identity,
            section::DATA_PERMISSION.for_kernel(),
        ),
    ];

    for item in section_def {
        let section = section::VirtualSection::new(item.0, item.1, item.2, item.3);
        for (vpn, entry, _) in section.iter() {
            kernel_page_table_tree.map(vpn, entry);
        }
    }
    kernel_page_table_tree.active(); //真正启用地址映射
}
