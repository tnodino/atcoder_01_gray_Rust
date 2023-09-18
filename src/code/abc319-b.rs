// https://atcoder.jp/contests/abc319/tasks/abc319_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    for i in 0..=N {
        let mut c = '-';
        for j in 1..10 {
            if N % j == 0 && i % (N / j) == 0 {
                c = ((j as u8) + ('0' as u8)) as char;
                break;
            }
        }
        print!("{}", c);
    }
    println!();
}