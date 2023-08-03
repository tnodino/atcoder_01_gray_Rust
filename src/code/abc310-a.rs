// https://atcoder.jp/contests/abc310/tasks/abc310_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: usize,
        Q: usize,
        D: [usize; N],
    }
    println!("{}", min(P, Q + D.iter().min().unwrap()));
}