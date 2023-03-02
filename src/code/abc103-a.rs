// https://atcoder.jp/contests/abc103/tasks/abc103_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A1: usize,
        A2: usize,
        A3: usize,
    }
    println!("{}", max(A1, max(A2, A3)) - min(A1, min(A2, A3)));
}