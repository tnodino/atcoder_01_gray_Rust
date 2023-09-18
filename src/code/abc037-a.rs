// https://atcoder.jp/contests/abc037/tasks/abc037_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (A, B, C): (usize, usize, usize),
    }
    println!("{}", C / min(A, B));
}