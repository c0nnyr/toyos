#![no_std]
#![feature(global_asm)]
#![feature(linkage)]

mod arch;
#[macro_use]
pub mod io;
mod panic;

#[no_mangle]
#[link_section = ".text._start"]
pub fn _start() {
    main(); // TODO 应该有个退出机制
}

#[no_mangle]
#[linkage = "weak"]
fn main() -> usize {
    panic!("never here");
}
