// https://atcoder.jp/contests/abc297/tasks/abc297_d

use proconio::input;
use proconio::fastout;
use std::mem::swap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut A: usize,
        mut B: usize,
    }
    if A < B {
        swap(&mut A, &mut B);
    }
    let mut ans = 0;
    while B > 0 {
        ans += A / B;
        A %= B;
        swap(&mut A, &mut B);
    }
    println!("{}", ans - 1);
}