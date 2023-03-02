// https://atcoder.jp/contests/abc099/tasks/abc099_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let X = b - a;
    println!("{}", X * (X + 1) / 2 - b);
}