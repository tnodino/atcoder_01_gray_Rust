// https://atcoder.jp/contests/abc209/tasks/abc209_c

use proconio::input;
use proconio::fastout;
use std::cmp::max;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut C: [usize; N],
    }
    C.sort();
    let mut ans = 1;
    for i in 0..N {
        ans *= max(0, C[i] - i);
        ans %= MOD;
    }
    println!("{}", ans);
}