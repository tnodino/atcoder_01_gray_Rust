// https://atcoder.jp/contests/abc142/tasks/abc142_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let odd = (N + 1) / 2;
    println!("{}", odd as f64 / N as f64);
}