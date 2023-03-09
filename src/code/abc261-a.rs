// https://atcoder.jp/contests/abc261/tasks/abc261_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        L1: isize,
        R1: isize,
        L2: isize,
        R2: isize,
    }
    println!("{}", max(0, min(R1, R2) - max(L1, L2)));
}