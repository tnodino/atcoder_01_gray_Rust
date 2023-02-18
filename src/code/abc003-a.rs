// https://atcoder.jp/contests/abc003/submissions/36298581

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = N as f64;
    let mut ans = 0.;
    for i in 1..=N {
        ans += (i * 10000) as f64 * (1. / M);
    }
    println!("{}", ans);
}