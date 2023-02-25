// https://atcoder.jp/contests/abc037/tasks/abc037_a

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
    let mi = min(A, B);
    println!("{}", C / mi);
}