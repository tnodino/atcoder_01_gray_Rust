// https://atcoder.jp/contests/abc244/tasks/abc244_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        S: String,
    }
    println!("{}", S.chars().last().unwrap());
}