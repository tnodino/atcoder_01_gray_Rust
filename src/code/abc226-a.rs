// https://atcoder.jp/contests/abc226/tasks/abc226_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: f64,
    }
    println!("{}", X.round());
}