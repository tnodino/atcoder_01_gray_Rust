// https://atcoder.jp/contests/abc156/tasks/abc156_c

use proconio::input;
use proconio::fastout;
use num::pow;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: [isize; N],
    }
    let mut ans = 1<<30;
    for p in 1..=100 {
        let mut cnt = 0;
        for i in 0..N {
            cnt += pow(X[i] - p, 2);
        }
        ans = min(ans, cnt);
    }
    println!("{}", ans);
}