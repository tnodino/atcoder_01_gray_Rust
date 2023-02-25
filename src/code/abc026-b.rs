// https://atcoder.jp/contests/abc026/tasks/abc026_b

use proconio::input;
use proconio::fastout;
use std::f64::consts::PI;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut R: [f64; N],
    }
    R.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut flg = N % 2;
    let mut ans = 0.;
    for r in R {
        if flg == 1 {
            ans += r * r;
        }
        else {
            ans -= r * r;
        }
        flg ^= 1;
    }
    println!("{}", ans * PI);
}