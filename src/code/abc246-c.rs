// https://atcoder.jp/contests/abc246/tasks/abc246_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, mut K, X): (usize, usize, usize),
        mut A: [usize; N],
    }
    for i in 0..N {
        let d = A[i] / X;
        if K <= d {
            A[i] -= X * K;
            K = 0;
            break;
        }
        A[i] -= d * X;
        K -= d;
    }
    A.sort_by(|a, b| b.cmp(&a));
    for i in 0..min(N, K) {
        A[i] = 0;
    }
    println!("{}", A.iter().sum::<usize>());
}