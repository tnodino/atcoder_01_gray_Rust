// https://atcoder.jp/contests/abc120/tasks/abc120_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let cnt = S.chars().filter(|&x| x == '0').count();
    println!("{}", min(cnt, S.len() - cnt) * 2);
}