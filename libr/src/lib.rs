#![no_std]
#![feature(global_asm)]
#![feature(linkage)]

mod arch;
#[macro_use]
pub mod io;
mod panic;
pub mod system;

#[no_mangle]
#[link_section = ".text._start"]
pub fn _start() {
    system::exit(main());
}

#[no_mangle]
#[linkage = "weak"]
fn main() -> usize {
    panic!("never here");
}
