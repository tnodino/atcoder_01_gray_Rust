// https://atcoder.jp/contests/abc095/tasks/abc095_b

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut X: usize,
    }
    let mut mi: usize = !0;
    for _ in 0..N {
        input! {
            m: usize,
        }
        X -= m;
        mi = min(mi, m);
    }
    println!("{}", N + X / mi);
}