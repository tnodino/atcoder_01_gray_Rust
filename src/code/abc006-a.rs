// https://atcoder.jp/contests/abc006/tasks/abc006_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", match N % 3 {
        0 => "YES",
        _ => "NO",
    });
}