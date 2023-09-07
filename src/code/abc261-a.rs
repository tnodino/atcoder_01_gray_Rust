// https://atcoder.jp/contests/abc261/tasks/abc261_a

use proconio::input;
use proconio::fastout;
use std::cmp::{max, min};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (L1, R1, L2, R2): (isize, isize, isize, isize),
    }
    println!("{}", max(0, min(R1, R2) - max(L1, L2)));
}