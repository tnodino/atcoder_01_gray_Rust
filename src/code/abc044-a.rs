// https://atcoder.jp/contests/abc044/tasks/abc044_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        X: usize,
        Y: usize,
    }
    if N <= K {
        println!("{}", N * X);
    }
    else {
        println!("{}", K * X + (N - K) * Y);
    }
}