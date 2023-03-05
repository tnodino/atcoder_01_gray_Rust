// https://atcoder.jp/contests/abc227/tasks/abc227_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: usize,
    }
    let mut ans = (A + K - 1) % N;
    if ans == 0 {
        ans += N;
    }
    println!("{}", ans);
}