// https://atcoder.jp/contests/abc053/tasks/abc053_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        x: usize,
    }
    if x < 1200 {
        println!("ABC");
    }
    else {
        println!("ARC");
    }
}