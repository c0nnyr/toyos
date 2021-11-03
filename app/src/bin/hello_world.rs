#![no_std]
#![no_main]

use libr::println;
use libr::system;
#[no_mangle]
fn main() {
    println!("Hello world 1 at {:?}!", system::get_now());
    // system::sleep(core::time::Duration::from_secs(2));
    println!("Hello world 2 at {:?}!", system::get_now());
}
