// https://atcoder.jp/contests/abc072/tasks/abc072_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: isize,
        t: isize,
    }
    println!("{}", max(0, X - t));
}