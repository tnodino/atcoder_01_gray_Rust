// https://atcoder.jp/contests/arc008/tasks/arc008_1

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans: usize = 1<<60;
    for x in 0..=50 {
        for y in 0..=5 {
            if x + y * 10 >= N {
                ans = min(ans, x * 15 + y * 100);
            }
        }
    }
    println!("{}", ans);
}