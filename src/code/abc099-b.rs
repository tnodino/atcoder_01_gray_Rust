// https://atcoder.jp/contests/abc099/tasks/abc099_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (a, b): (usize, usize),
    }
    let s = b - a;
    println!("{}", (s + 1) * s / 2 - b);
}