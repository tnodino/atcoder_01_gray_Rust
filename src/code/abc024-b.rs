// https://atcoder.jp/contests/abc024/tasks/abc024_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, T): (usize, usize),
        A: [usize; N],
    }
    let mut ans = T * N;
    for i in 1..N {
        if A[i] < A[i-1] + T {
            ans -= T - (A[i] - A[i-1]);
        }
    }
    println!("{}", ans);
}