// https://atcoder.jp/contests/abc144/tasks/abc144_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    if A >= 10 || B >= 10 {
        println!("-1");
    }
    else {
        println!("{}", A * B);
    }
}