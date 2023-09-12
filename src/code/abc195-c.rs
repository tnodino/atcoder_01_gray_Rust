// https://atcoder.jp/contests/abc195/tasks/abc195_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = 0;
    if N >= 1_000 {
        ans += N - 999;
    }
    if N >= 1_000_000 {
        ans += N - 999_999;
    }
    if N >= 1_000_000_000 {
        ans += N - 999_999_999;
    }
    if N >= 1_000_000_000_000 {
        ans += N - 999_999_999_999;
    }
    if N >= 1_000_000_000_000_000 {
        ans += N - 999_999_999_999_999;
    }
    println!("{}", ans);
}