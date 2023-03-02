// https://atcoder.jp/contests/abc130/tasks/abc130_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        A: usize,
    }
    if X < A {
        println!("0");
    }
    else {
        println!("10");
    }
}