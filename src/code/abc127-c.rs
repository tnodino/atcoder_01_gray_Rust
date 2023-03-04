// https://atcoder.jp/contests/abc127/tasks/abc127_c

use proconio::input;
use proconio::fastout;
use std::cmp::max;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: isize,
        M: usize,
    }
    let mut l = 0;
    let mut r = N;
    for _ in 0..M {
        input! {
            L: isize,
            R: isize,
        }
        l = max(l, L);
        r = min(r, R);
    }
    println!("{}", max(0, r - l + 1));
}