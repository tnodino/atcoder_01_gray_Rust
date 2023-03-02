// https://atcoder.jp/contests/abc120/tasks/abc120_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
    }
    println!("{}", min(B / A, C));
}