// https://atcoder.jp/contests/arc123/tasks/arc123_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A1: isize,
        A2: isize,
        A3: isize,
    }
    let A = A2 * 2 - A1 - A3;
    let d = max(0, (-A + 1) / 2);
    println!("{}", d + (A + 2 * d))
}