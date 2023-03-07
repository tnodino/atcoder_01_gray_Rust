// https://atcoder.jp/contests/abc144/tasks/abc144_c

use proconio::input;
use proconio::fastout;
use libm::sqrt;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = sqrt(N as f64) as usize;
    let mut ans: usize = !0;
    for i in 1..=M {
        if N % i == 0 {
            ans = min(ans, i + (N / i) - 2);
        }
    }
    println!("{}", ans);
}