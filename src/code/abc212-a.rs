// https://atcoder.jp/contests/abc212/tasks/abc212_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    if 0 < A && B == 0 {
        println!("Gold");
    }
    else if 0 == A && 0 < B {
        println!("Silver");
    }
    else {
        println!("Alloy");
    }
}