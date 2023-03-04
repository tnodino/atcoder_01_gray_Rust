// https://atcoder.jp/contests/abc143/tasks/abc143_a

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
    println!("{}", max(0, A - B * 2));
}