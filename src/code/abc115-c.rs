// https://atcoder.jp/contests/abc115/tasks/abc115_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        mut h: [usize; N],
    }
    h.sort();
    let mut ans: usize = !0;
    for i in 0..=N-K {
        ans = min(ans, h[i+K-1] - h[i]);
    }
    println!("{}", ans);
}