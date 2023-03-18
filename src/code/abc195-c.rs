// https://atcoder.jp/contests/abc195/tasks/abc195_c

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: isize,
    }
    let mut ans = 0;
    ans += max(0, N - 999);
    ans += max(0, N - 999_999);
    ans += max(0, N - 999_999_999);
    ans += max(0, N - 999_999_999_999);
    ans += max(0, N - 999_999_999_999_999);
    println!("{}", ans);
}