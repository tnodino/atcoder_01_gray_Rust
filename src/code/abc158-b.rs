// https://atcoder.jp/contests/abc158/tasks/abc158_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
        A: usize,
        B: usize,
    }
    let S = A + B;
    let mut ans = N / S * A;
    N %= S;
    if N <= A {
        ans += N;
    }
    else {
        ans += A;
    }
    println!("{}", ans);
}