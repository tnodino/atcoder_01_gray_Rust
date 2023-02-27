// https://atcoder.jp/contests/abc291/tasks/abc291_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut X: [f64; 5 * N],
    }
    X.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut ans = 0.;
    for i in N..4*N {
        ans += X[i];
    }
    println!("{}", ans / (N as f64 * 3.));
}