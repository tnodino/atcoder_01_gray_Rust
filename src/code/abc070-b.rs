// https://atcoder.jp/contests/abc070/tasks/abc070_b

use proconio::input;
use proconio::fastout;
use std::cmp::max;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: isize,
        B: isize,
        C: isize,
        D: isize,
    }
    println!("{}", max(0, min(B, D) - max(A, C)));
}