// https://atcoder.jp/contests/abc099/tasks/abc099_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", match N <= 999 {
        true => "ABC",
        false => "ABD",
    });
}