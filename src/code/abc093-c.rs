// https://atcoder.jp/contests/abc093/tasks/arc094_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
    }
    let M = max(A, max(B, C));
    let M = 3 * M - A - B - C;
    if M % 2 == 0 {
        println!("{}", M / 2);
    }
    else {
        println!("{}", M / 2 + 2);
    }
}