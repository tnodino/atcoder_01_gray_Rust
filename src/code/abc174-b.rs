// https://atcoder.jp/contests/abc174/tasks/abc174_b

use proconio::input;
use proconio::fastout;
use libm::hypot;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        D: f64,
    }
    let mut ans = 0;
    for _ in 0..N {
        input! {
            X: f64,
            Y: f64,
        }
        if hypot(X, Y) <= D {
            ans += 1;
        }
    }
    println!("{}", ans);
}