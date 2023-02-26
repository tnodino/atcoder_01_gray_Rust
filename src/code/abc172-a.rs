// https://atcoder.jp/contests/abc172/tasks/abc172_a

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
    }
    println!("{}", a + pow(a, 2) + pow(a, 3));
}