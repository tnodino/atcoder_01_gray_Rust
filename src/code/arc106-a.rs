// https://atcoder.jp/contests/arc106/tasks/arc106_a

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    for a in 1..=37 {
        for b in 1..=25 {
            if pow(3usize, a) + pow(5usize, b) == N {
                println!("{} {}", a, b);
                return;
            }
        }
    }
    println!("-1");
}