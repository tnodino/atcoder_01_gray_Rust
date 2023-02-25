// https://atcoder.jp/contests/abc024/tasks/abc024_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut A: usize,
        mut B: usize,
        C: usize,
        K: usize,
        S: usize,
        T: usize,
    }
    if S + T >= K {
        A -= C;
        B -= C;
    }
    println!("{}", A * S + B * T);
}