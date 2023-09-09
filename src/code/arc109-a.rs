// https://atcoder.jp/contests/arc109/tasks/arc109_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (a, b, x, y): (isize, isize, isize, isize),
    }
    let y = min(x * 2, y);
    let d = (b * 2 + 1 - a * 2).abs();
    println!("{}", (d / 2 * y) + x);
}