// https://atcoder.jp/contests/abc320/tasks/abc320_a

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (A, B): (usize, usize),
    }
    println!("{}", pow(A, B) + pow(B, A));
}