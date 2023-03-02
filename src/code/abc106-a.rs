// https://atcoder.jp/contests/abc106/tasks/abc106_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    println!("{}", A * B - A - B + 1);
}