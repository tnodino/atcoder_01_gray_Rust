// https://atcoder.jp/contests/abc041/tasks/abc041_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String,
        i: usize,
    }
    println!("{}", s.chars().nth(i-1).unwrap());
}