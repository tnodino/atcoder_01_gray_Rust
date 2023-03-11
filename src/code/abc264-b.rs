// https://atcoder.jp/contests/abc264/tasks/abc264_b

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        R: isize,
        C: isize,
    }
    let ma = max((R - 8).abs(), (C - 8).abs());
    if ma % 2 == 0 {
        println!("white");
    }
    else {
        println!("black");
    }
}