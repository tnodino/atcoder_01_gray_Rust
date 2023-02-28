// https://atcoder.jp/contests/abc086/tasks/abc086_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    println!("{}", match a * b % 2 {
        1 => "Odd",
        _ => "Even",
    });
}