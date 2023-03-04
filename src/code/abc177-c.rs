// https://atcoder.jp/contests/abc177/tasks/abc177_c

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut sum = vec![0; N];
    for i in (0..N-1).rev() {
        sum[i] += A[i+1] + sum[i+1];
        sum[i] %= MOD;
    }
    let mut ans = 0;
    for i in 0..N {
        ans += A[i] * sum[i];
        ans %= MOD;
    }
    println!("{}", ans);
}