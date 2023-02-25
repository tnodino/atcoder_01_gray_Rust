// https://atcoder.jp/contests/abc013/tasks/abc013_2

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: isize,
        b: isize,
    }
    let mut now = a as isize;
    let mut red = 0;
    while now != b {
        now += 1;
        now %= 10;
        red += 1;
    }
    let mut now = a as isize;
    let mut blue = 0;
    while now != b {
        now -= 1;
        now += 10;
        now %= 10;
        blue += 1;
    }
    println!("{}", min(red, blue));
}