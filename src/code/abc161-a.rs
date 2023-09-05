// https://atcoder.jp/contests/abc161/tasks/abc161_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (X, Y, Z): (usize, usize, usize),
    }
    println!("{} {} {}", Z, X, Y);
}