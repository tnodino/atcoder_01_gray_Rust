// https://atcoder.jp/contests/abc159/tasks/abc159_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    println!("{}", (N - 1) * N / 2 + (M - 1) * M / 2);
}