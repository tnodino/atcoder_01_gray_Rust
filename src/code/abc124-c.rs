// https://atcoder.jp/contests/abc124/tasks/abc124_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut black = 0;
    for (i, s) in S.chars().enumerate() {
        if i % 2 == 0 {
            if s == '1' {
                black += 1;
            }
        }
        else {
            if s == '0' {
                black += 1;
            }
        }
    }
    let mut white = 0;
    for (i, s) in S.chars().enumerate() {
        if i % 2 == 0 {
            if s == '0' {
                white += 1;
            }
        }
        else {
            if s == '1' {
                white += 1;
            }
        }
    }
    println!("{}", min(black, white));
}