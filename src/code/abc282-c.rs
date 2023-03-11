// https://atcoder.jp/contests/abc282/tasks/abc282_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut flg = 0;
    for s in S {
        let mut t = s;
        if s == '"' {
            flg ^= 1;
        }
        else if s == ',' && flg == 0 {
            t = '.';
        }
        print!("{}", t);
    }
    println!();
}