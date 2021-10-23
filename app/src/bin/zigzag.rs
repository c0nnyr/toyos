#![no_std]
#![no_main]

use libr::println;
#[no_mangle]
fn main() {
    let arr = [
        [1, 2, 3, 4, 5],      //
        [7, 8, 9, 10, 11],    //
        [12, 13, 14, 15, 16], //
    ];
    let max_x = arr[0].len() as i32 - 1;
    let max_y = arr.len() as i32 - 1;
    println!("Input array:");
    for y in 0..=max_y {
        println!(
            "{} {} {} {} {}",
            arr[y as usize][0],
            arr[y as usize][1],
            arr[y as usize][2],
            arr[y as usize][3],
            arr[y as usize][4] //五列数据
        );
    }
    println!("Zigzag is:");
    for i in 0..=(max_x + max_y) {
        let left_x1 = 0;
        let left_y1 = i;
        let left_x2 = i - max_y;
        let left_y2 = max_y;
        let right_x1 = max_x;
        let right_y1 = i - max_x;
        let right_x2 = i;
        let right_y2 = 0;

        let left_x = left_x2.max(left_x1);
        let left_y = left_y2.min(left_y1);
        let right_x = right_x2.min(right_x1);
        let right_y = right_y2.max(right_y1);
        if i % 2 == 0 {
            for x in left_x..=right_x {
                let y = left_y - (x - left_x);
                println!("{} ", arr[y as usize][x as usize]);
            }
        } else {
            for x in (left_x..=right_x).rev() {
                let y = right_y + (right_x - x);
                println!("{} ", arr[y as usize][x as usize]);
            }
        }
    }
}
