// https://atcoder.jp/contests/abc146/tasks/abc146_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: u8,
        S: String,
    }
    for s in S.chars() {
        let mut c = s as u8 - 65;
        c += N;
        c %= 26;
        print!("{}", (c + 65) as char);
    }
    println!();
}