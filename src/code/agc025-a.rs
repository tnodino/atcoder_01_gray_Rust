// https://atcoder.jp/contests/agc025/tasks/agc025_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

fn sum_digit(mut x: usize) -> usize {
    let mut ret = 0;
    while x > 0 {
        ret += x % 10;
        x /= 10;
    }
    return ret
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans: usize = !0;
    for a in 1..N {
        let b = N - a;
        ans = min(ans, sum_digit(a) + sum_digit(b));
    }
    println!("{}", ans);
}