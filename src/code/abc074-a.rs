// https://atcoder.jp/contests/abc074/tasks/abc074_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
    }
    println!("{}", N * N - A)
}