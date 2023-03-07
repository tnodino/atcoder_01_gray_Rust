// https://atcoder.jp/contests/abc143/tasks/abc143_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        S: String,
    }
    let mut S = S.chars().collect::<Vec<char>>();
    S.dedup();
    println!("{}", S.len());
}