// https://atcoder.jp/contests/abc148/tasks/abc148_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    println!("{}", 6 - A - B);
}