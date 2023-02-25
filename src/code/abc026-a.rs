// https://atcoder.jp/contests/abc026/tasks/abc026_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
    }
    let mut ans = 0;
    for x in 1..A {
        let y = A - x;
        ans = max(ans, x * y);
    }
    println!("{}", ans);
}