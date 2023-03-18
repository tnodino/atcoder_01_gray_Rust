// https://atcoder.jp/contests/arc112/tasks/arc112_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
    }
    for _ in 0..T {
        input! {
            L: isize,
            R: isize,
        }
        let ma = max(0, R - L * 2 + 1);
        println!("{}", (ma + 1) * ma / 2);
    }
}