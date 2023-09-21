// https://atcoder.jp/contests/abc040/tasks/abc040_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (n, x): (usize, usize),
    }
    println!("{}", min(n - x, x - 1));
}