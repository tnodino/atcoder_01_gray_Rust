// https://atcoder.jp/contests/abc138/tasks/abc138_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [f64; N],
    }
    let mut ans = 0.;
    for i in 0..N {
        ans += 1. / A[i];
    }
    println!("{}", 1. / ans);
}