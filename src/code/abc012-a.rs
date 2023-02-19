// https://atcoder.jp/contests/abc012/tasks/abc012_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    println!("{} {}", B, A);
}