// https://atcoder.jp/contests/abc100/tasks/abc100_b

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        D: usize,
        mut N: usize,
    }
    if N == 100 {
        N += 1;
    }
    println!("{}", pow(100, D) * N);
}