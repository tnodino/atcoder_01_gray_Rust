// https://atcoder.jp/contests/abc138/tasks/abc138_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut v: [f64; N],
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut ans = v[0];
    for i in 1..N {
        ans += v[i];
        ans /= 2.;
    }
    println!("{}", ans);
}