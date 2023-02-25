// https://atcoder.jp/contests/abc040/tasks/abc040_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        x: usize,
    }
    println!("{}", min(x - 1, n - x));
}