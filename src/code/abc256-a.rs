// https://atcoder.jp/contests/abc256/tasks/abc256_a

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", pow(2, N));
}