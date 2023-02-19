// https://atcoder.jp/contests/abc012/tasks/abc012_3

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = 2025 - N;
    for i in 1..=9 {
        for j in 1..=9 {
            if i * j == M {
                println!("{} x {}", i, j);
            }
        }
    }
}