// https://atcoder.jp/contests/abc031/tasks/abc031_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        D: usize,
    }
    println!("{}", max((A + 1) * D, A * (D + 1)));
}