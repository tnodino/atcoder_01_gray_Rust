// https://atcoder.jp/contests/abc151/tasks/abc151_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        C: char,
    }
    println!("{}", (C as u8 + 1) as char);
}