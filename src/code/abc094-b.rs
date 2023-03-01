// https://atcoder.jp/contests/abc094/tasks/abc094_b

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        X: usize,
        A: [usize; M],
    }
    let mut cost = vec![0; N+1];
    for i in 0..M {
        cost[A[i]] = 1;
    }
    for i in (0..X).rev() {
        cost[i] += cost[i+1];
    }
    for i in X+1..=N {
        cost[i] += cost[i-1];
    }
    println!("{}", min(cost[0], cost[N]));
}