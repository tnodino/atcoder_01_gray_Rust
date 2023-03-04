// https://atcoder.jp/contests/abc127/tasks/abc127_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        mut B: usize,
    }
    B = match A {
        v if v <= 5 => 0,
        v if v <= 12 => B / 2,
        _ => B,
    };
    println!("{}", B);
}