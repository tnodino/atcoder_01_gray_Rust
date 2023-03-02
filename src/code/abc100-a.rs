// https://atcoder.jp/contests/abc100/tasks/abc100_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    if A <= 8 && B <= 8 {
        println!("Yay!");
    }
    else {
        println!(":(");
    }
}