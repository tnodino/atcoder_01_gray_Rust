// https://atcoder.jp/contests/abc039/tasks/abc039_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize
    }
    println!("{}", (A * B + A * C + B * C) * 2);
}