pub mod addr;
pub mod page_table;
pub mod ppn_manager;

pub fn init(){
    ppn_manager::init();
    init_kernel_map();
}

//总需要个地方Own这个映射，否则申请的内存页都被释放了
//暂时放在这里
static KERNEL_PAGE_TABLE_TREE : spin::Mutex<page_table::PageTableTree> = spin::Mutex::new(page_table::PageTableTree::default());

fn init_kernel_map(){
    let kernel_page_table_tree = &mut KERNEL_PAGE_TABLE_TREE.lock();
    kernel_page_table_tree.init().unwrap();
    for i in 0x80200..0x80900{
        let entry = page_table::PageTableEntry {
            ppn: addr::PhysicalPageNumber::from(i),
            valid: true,
            read: true,
            write: true,
            execute: true,
            user: false,
        };
        kernel_page_table_tree.map(addr::VirtualPageNumber::from(i), entry).unwrap();
    }
    kernel_page_table_tree.active(); //真正启用地址映射
}