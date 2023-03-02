// https://atcoder.jp/contests/abc140/tasks/abc140_a

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", pow(N, 3));
}