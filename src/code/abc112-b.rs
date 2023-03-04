// https://atcoder.jp/contests/abc112/tasks/abc112_b

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        T: usize,
    }
    let mut ans: usize = !0;
    for _ in 0..N {
        input! {
            c: usize,
            t: usize,
        }
        if t <= T {
            ans = min(ans, c);
        }
    }
    if ans == !0 as usize {
        println!("TLE");
    }
    else {
        println!("{}", ans);
    }
}