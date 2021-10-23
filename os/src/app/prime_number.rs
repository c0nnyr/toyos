pub fn main() {
    const N: usize = 100;
    let mut arr: [usize; N + 1] = [0; N + 1];
    for i in 0..=N {
        arr[i] = i;
    }
    arr[0] = 0;
    arr[1] = 0;
    for i in 3..=N {
        for j in 2..i {
            if arr[j] != 0 && i % j == 0 {
                arr[i] = 0; // not prime number
            }
        }
    }
    kinfo!("Prime number from 0 .. {} is:", N);
    for i in 0..=N {
        if arr[i] != 0 {
            kinfo!("{}", arr[i]);
        }
    }
}
