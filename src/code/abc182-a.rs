// https://atcoder.jp/contests/abc182/tasks/abc182_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    println!("{}", (A * 2 + 100) - B);
}