// https://atcoder.jp/contests/abc248/tasks/abc248_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut A: usize,
        B: usize,
        K: usize,
    }
    let mut ans = 0;
    while A < B {
        A *= K;
        ans += 1;
    }
    println!("{}", ans);
}