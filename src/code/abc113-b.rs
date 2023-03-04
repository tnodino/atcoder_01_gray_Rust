// https://atcoder.jp/contests/abc113/tasks/abc113_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        T: f64,
        A: f64,
        H: [f64; N],
    }
    let mut C = vec![0.; N];
    for (i, h) in H.iter().enumerate() {
        C[i] = T - h * 0.006;
    }
    let mut near = (H[0] - A).abs();
    let mut ans = 1;
    for (i, c) in C.iter().enumerate() {
        if (c - A).abs() < near {
            near = (c - A).abs();
            ans = i + 1;
        }
    }
    println!("{}", ans);
}