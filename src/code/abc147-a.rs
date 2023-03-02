// https://atcoder.jp/contests/abc147/tasks/abc147_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A1: usize,
        A2: usize,
        A3: usize,
    }
    if A1 + A2 + A3 >= 22 {
        println!("bust");
    }
    else {
        println!("win");
    }
}