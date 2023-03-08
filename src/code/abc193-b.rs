// https://atcoder.jp/contests/abc193/tasks/abc193_b

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans: usize = !0;
    for _ in 0..N {
        input! {
            A: usize,
            P: usize,
            X: usize,
        }
        if A < X {
            ans = min(ans, P);
        }
    }
    if ans == !0 as usize {
        println!("-1");
    }
    else {
        println!("{}", ans);
    }
}