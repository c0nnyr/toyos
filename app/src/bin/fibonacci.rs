#![no_std]
#![no_main]

use libr::{print, println, system};
#[no_mangle]
fn main() {
    const N: usize = 20;
    let mut arr: [usize; N] = [0; N];
    arr[0] = 1;
    arr[1] = 1;
    for i in 2..N {
        arr[i] = arr[i - 1] + arr[i - 2];
    }
    system::sleep(core::time::Duration::from_secs(2));
    println!("Fibonacci number from 1 .. {} is:", N);
    for i in 0..N {
        print!("{}:{} ", i + 1, arr[i]);
    }
    println!("Fibonacci number done");
}
