// https://atcoder.jp/contests/abc210/tasks/abc210_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        S: String,
    }
    for (i, s) in S.chars().enumerate() {
        if i % 2 == 0 {
            if s == '1' {
                println!("Takahashi");
                return;
            }
        }
        else {
            if s == '1' {
                println!("Aoki");
                return;
            }
        }
    }
}