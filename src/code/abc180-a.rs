// https://atcoder.jp/contests/abc180/tasks/abc180_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
    }
    println!("{}", N - A + B);
}