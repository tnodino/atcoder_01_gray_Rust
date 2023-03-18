// https://atcoder.jp/contests/arc116/tasks/arc116_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
    }
    for _ in 0..T {
        input! {
            N: usize,
        }
        if N % 4 == 0 {
            println!("Even");
        }
        else if N % 2 == 0 {
            println!("Same");
        }
        else {
            println!("Odd");
        }
    }
}