// https://atcoder.jp/contests/abc140/tasks/abc140_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        B: [usize; N-1],
    }
    let mut A: Vec<usize> = vec![!0; N];
    for i in 0..N-1 {
        A[i] = min(A[i], B[i]);
        A[i+1] = min(A[i+1], B[i]);
    }
    println!("{}", A.iter().sum::<usize>());
}