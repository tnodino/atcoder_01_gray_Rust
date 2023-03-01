// https://atcoder.jp/contests/abc128/tasks/abc128_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        P: usize,
    }
    println!("{}", (A * 3 + P) / 2);
}