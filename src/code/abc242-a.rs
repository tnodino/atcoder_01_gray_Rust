// https://atcoder.jp/contests/abc242/tasks/abc242_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: f64,
        B: f64,
        C: f64,
        X: f64,
    }
    if X <= A {
        println!("1");
    }
    else if B < X {
        println!("0");
    }
    else {
        println!("{}", C / (B - A));
    }
}