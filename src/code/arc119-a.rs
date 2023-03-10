// https://atcoder.jp/contests/arc119/tasks/arc119_a

use proconio::input;
use proconio::fastout;
use num::pow;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = N;
    for b in 0..=60 {
        if pow(2, b) > N {
            break;
        }
        let po = pow(2, b);
        let a = N / po;
        let c = N - a * po;
        ans = min(ans, a + b + c);
    }
    println!("{}", ans);
}