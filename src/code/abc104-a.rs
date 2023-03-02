// https://atcoder.jp/contests/abc104/tasks/abc104_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        R: usize,
    }
    println!("{}", match R {
        v if v < 1200 => "ABC",
        v if v < 2800 => "ARC",
        _ => "AGC",
    });
}