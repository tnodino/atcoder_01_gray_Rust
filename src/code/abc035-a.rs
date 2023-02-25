// https://atcoder.jp/contests/abc035/tasks/abc035_a

use proconio::input;
use proconio::fastout;
use num::integer::gcd;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        W: usize,
        H: usize,
    }
    let g = gcd(W, H);
    println!("{}:{}", W / g, H / g);
}