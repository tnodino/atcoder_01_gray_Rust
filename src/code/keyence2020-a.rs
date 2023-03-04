// https://atcoder.jp/contests/keyence2020/tasks/keyence2020_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        N: usize,
    }
    let M = max(H, W);
    println!("{}", (N + M - 1) / M);
}