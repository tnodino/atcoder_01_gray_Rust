// https://atcoder.jp/contests/abc024/tasks/abc024_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        T: usize,
    }
    input! {
        mut now: usize,
    }
    let mut ans = T;
    for _ in 0..N-1 {
        input! {
            A: usize,
        }
        if A - now < T {
            ans += A - now;
        }
        else {
            ans += T
        }
        now = A;
    }
    println!("{}", ans);
}