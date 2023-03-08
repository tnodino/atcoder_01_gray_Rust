// https://atcoder.jp/contests/arc131/tasks/arc131_a

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    println!("{}", A * pow(10, 9) + B * 10 / 2);
}