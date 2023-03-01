// https://atcoder.jp/contests/abc233/tasks/abc233_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: isize,
        Y: isize,
    }
    println!("{}", max(0, (Y - X + 9) / 10));
}