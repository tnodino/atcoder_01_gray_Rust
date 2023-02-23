// https://atcoder.jp/contests/abc005/tasks/abc005_2

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = 100;
    for _ in 0..N {
        input! {
            T: usize,
        }
        ans = min(ans, T);
    }
    println!("{}", ans);
}