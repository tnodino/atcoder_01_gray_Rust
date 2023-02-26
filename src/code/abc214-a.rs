// https://atcoder.jp/contests/abc214/tasks/abc214_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    if N <= 125 {
        println!("4");
    }
    else if N <= 211 {
        println!("6");
    }
    else {
        println!("8");
    }
}