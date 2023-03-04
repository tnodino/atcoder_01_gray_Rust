// https://atcoder.jp/contests/abc112/tasks/abc112_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    if N == 1 {
        println!("Hello World");
    }
    else {
        input! {
            A: usize,
            B: usize,
        }
        println!("{}", A + B);
    }
}