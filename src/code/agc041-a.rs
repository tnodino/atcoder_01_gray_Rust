// https://atcoder.jp/contests/agc041/tasks/agc041_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
    }
    if A % 2 == B % 2 {
        println!("{}", (B - A) / 2);
    }
    else {
        println!("{}", min(A - 1, N - B) + 1 + (B - A - 1) / 2);
    }
}