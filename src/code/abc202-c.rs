// https://atcoder.jp/contests/abc202/tasks/abc202_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        B: [usize; N],
        C: [usize; N],
    }
    let mut acnt = vec![0usize; N+1];
    let mut bcnt = vec![0usize; N+1];
    for i in 0..N {
        acnt[A[i]] += 1;
        bcnt[B[C[i]-1]] += 1;
    }
    let mut ans = 0;
    for i in 1..=N {
        ans += acnt[i] * bcnt[i];
    }
    println!("{}", ans);
}