// https://atcoder.jp/contests/abc221/tasks/abc221_a

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
    println!("{}", pow(32, A - B));
}