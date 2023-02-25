// https://atcoder.jp/contests/abc034/tasks/abc034_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        x: usize,
        y: usize,
    }
    if x < y {
        println!("Better");
    }
    else {
        println!("Worse");
    }
}