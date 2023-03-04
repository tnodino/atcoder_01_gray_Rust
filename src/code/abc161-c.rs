// https://atcoder.jp/contests/abc161/tasks/abc161_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: isize,
        K: isize,
    }
    let ans = N % K;
    println!("{}", min(ans, (ans - K).abs()));
}