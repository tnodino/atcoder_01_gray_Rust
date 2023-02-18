// https://atcoder.jp/contests/abc002/tasks/abc002_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        Y: usize,
    }
    println!("{}", X.max(Y));
}