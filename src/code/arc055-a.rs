// https://atcoder.jp/contests/arc055/tasks/arc055_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    print!("1");
    for _ in 0..N-1 {
        print!("0");
    }
    println!("7");
}