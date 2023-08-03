// https://atcoder.jp/contests/abc311/tasks/abc311_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        S: String,
    }
    println!("{}", max(S.find("A").unwrap(), max(S.find("B").unwrap(), S.find("C").unwrap())) + 1);
}