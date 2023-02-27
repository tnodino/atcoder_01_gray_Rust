// https://atcoder.jp/contests/abc181/tasks/abc181_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    if N & 1 == 0 {
        println!("White");
    }
    else {
        println!("Black");
    }
}