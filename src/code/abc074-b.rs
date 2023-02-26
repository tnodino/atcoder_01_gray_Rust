// https://atcoder.jp/contests/abc074/tasks/abc074_b

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        x: [usize; N]
    }
    let mut ans = 0;
    for i in 0..N {
        ans += min(x[i] * 2, (K - x[i]) * 2);
    }
    println!("{}", ans);
}