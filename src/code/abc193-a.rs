// https://atcoder.jp/contests/abc193/tasks/abc193_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: f64,
        B: f64,
    }
    println!("{}", (1. - B / A) * 100.);
}