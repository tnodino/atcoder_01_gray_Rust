// https://atcoder.jp/contests/abc088/tasks/abc088_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
    }
    if N % 500 <= A {
        println!("Yes");
    }
    else {
        println!("No");
    }
}