// https://atcoder.jp/contests/abc098/tasks/abc098_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: isize,
        B: isize,
    }
    println!("{}", max(A + B, max(A - B, A * B)));
}