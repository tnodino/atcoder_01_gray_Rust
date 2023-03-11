// https://atcoder.jp/contests/abc269/tasks/abc269_b

use proconio::input;
use proconio::fastout;
use std::cmp::max;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let mut A = 10;
    let mut B = 0;
    let mut C = 10;
    let mut D = 0;
    for i in 0..10 {
        input! {
            S: String,
        }
        let S = S.chars().collect::<Vec<char>>();
        for j in 0..10 {
            if S[j] == '#' {
                A = min(A, i + 1);
                B = max(B, i + 1);
                C = min(C, j + 1);
                D = max(D, j + 1);
            }
        }
    }
    println!("{} {}\n{} {}", A, B, C, D);
}