// https://atcoder.jp/contests/abc210/tasks/abc210_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
        X: usize,
        Y: usize,
    }
    if N <= A {
        println!("{}", N * X);
    }
    else {
        println!("{}", A * X + (N - A) * Y);
    }
}