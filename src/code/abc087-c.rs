// https://atcoder.jp/contests/abc087/tasks/arc090_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [[usize; N]; 2],
    }
    let mut DP = vec![vec![0; N]; 2];
    DP[0][0] = A[0][0];
    for i in 0..2 {
        for j in 0..N {
            if i == 0 && j == 0 {
                continue;
            }
            if i > 0 {
                DP[i][j] = max(DP[i][j], DP[i-1][j] + A[i][j]);
            }
            if j > 0 {
                DP[i][j] = max(DP[i][j], DP[i][j-1] + A[i][j]);
            }
        }
    }
    println!("{}", DP[1][N-1]);
}