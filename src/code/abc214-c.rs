// https://atcoder.jp/contests/abc214/tasks/abc214_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: [usize; N],
        mut T: [usize; N],
    }
    for i in 0..N*2 {
        T[(i + 1) % N] = min(T[(i + 1) % N], T[i % N] + S[i % N]);
    }
    for t in T {
        println!("{}", t);
    }
}