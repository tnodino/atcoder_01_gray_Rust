// https://atcoder.jp/contests/agc020/tasks/agc020_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        A: usize,
        B: usize,
    }
    println!("{}", match (B - A - 1) % 2 {
        1 => "Alice",
        _ => "Borys",
    });
}