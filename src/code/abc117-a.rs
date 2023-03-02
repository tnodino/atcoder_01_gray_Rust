// https://atcoder.jp/contests/abc117/tasks/abc117_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        T: f64,
        X: f64,
    }
    println!("{}", T / X);
}