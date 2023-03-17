// https://atcoder.jp/contests/arc109/tasks/arc109_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: isize,
        b: isize,
        x: isize,
        mut y: isize,
    }
    y = min(x * 2, y);
    let d = (b * 2 - a * 2 + 1).abs();
    if d % 2 == 0 {
        println!("{}", d / 2 * y);
    }
    else {
        println!("{}", d / 2 * y + x);
    }
}