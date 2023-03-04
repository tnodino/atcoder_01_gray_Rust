// https://atcoder.jp/contests/abc176/tasks/abc176_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        T: usize,
    }
    println!("{}", (N + X - 1) / X * T);
}