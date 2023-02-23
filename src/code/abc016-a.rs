// https://atcoder.jp/contests/abc016/tasks/abc016_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        M: usize,
        D: usize,
    }
    println!("{}", match M % D {
        0 => "YES",
        _ => "NO",
    });
}