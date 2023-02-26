// https://atcoder.jp/contests/abc185/tasks/abc185_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A1: usize,
        A2: usize,
        A3: usize,
        A4: usize,
    }
    println!("{}", min(A1, min(A2, min(A3, A4))));
}