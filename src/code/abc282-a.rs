// https://atcoder.jp/contests/abc282/tasks/abc282_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
    }
    for i in 0..K {
        print!("{}", (i as u8 + 65) as char);
    }
    println!();
}