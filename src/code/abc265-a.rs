// https://atcoder.jp/contests/abc265/tasks/abc265_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        Y: usize,
        N: usize,
    }
    let A = N / 3 * Y + N % 3 * X;
    let B = N * X;
    println!("{}", min(A, B));
}