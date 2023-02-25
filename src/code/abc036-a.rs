// https://atcoder.jp/contests/abc036/tasks/abc036_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    println!("{}", (A + B - 1) / A);
}