// https://atcoder.jp/contests/abc152/tasks/abc152_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    if N == M {
        println!("Yes");
    }
    else {
        println!("No");
    }
}