// https://atcoder.jp/contests/abc223/tasks/abc223_b

use proconio::input;
use proconio::fastout;
use std::cmp::max;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut ma = S.clone();
    let mut mi = S.clone();
    let mut S = S.chars().collect::<Vec<char>>();
    for _ in 0..S.len() {
        S.rotate_left(1);
        ma = max(ma, S.iter().map(|&x| x.to_string()).collect::<String>());
        mi = min(mi, S.iter().map(|&x| x.to_string()).collect::<String>());
    }
    println!("{}\n{}", mi, ma);
}