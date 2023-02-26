// https://atcoder.jp/contests/abc204/tasks/abc204_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        x: usize,
        y: usize,
    }
    if x == y {
        println!("{}", x);
    }
    else {
        println!("{}", 3 ^ x ^ y);
    }
}