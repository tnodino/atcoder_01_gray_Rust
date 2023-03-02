// https://atcoder.jp/contests/abc108/tasks/abc108_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
    }
    println!("{}", ((K + 1) / 2) * (K / 2));
}