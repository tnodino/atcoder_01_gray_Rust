// https://atcoder.jp/contests/abc086/tasks/abc086_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (a, b): (usize, usize),
    }
    println!("{}", match (a * b) % 2 {
        0 => "Even",
        1 => "Odd",
        _ => unreachable!(),
    });
}