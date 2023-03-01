// https://atcoder.jp/contests/abc157/tasks/abc157_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", (N + 1) / 2);
}