// https://atcoder.jp/contests/abc136/tasks/abc136_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: isize,
        B: isize,
        C: isize,
    }
    println!("{}", max(0, C - (A - B)));
}