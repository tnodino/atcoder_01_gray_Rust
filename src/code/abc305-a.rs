// https://atcoder.jp/contests/abc305/tasks/abc305_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let n = N % 5;
    if 5 - n < n {
        println!("{}", N + 5 - n);
    }
    else {
        println!("{}", N - n);
    }
}