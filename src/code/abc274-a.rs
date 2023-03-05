// https://atcoder.jp/contests/abc274/tasks/abc274_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: f64,
        B: f64,
    }
    println!("{:.3}", B / A);
}