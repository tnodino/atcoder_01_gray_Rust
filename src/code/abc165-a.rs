// https://atcoder.jp/contests/abc165/tasks/abc165_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
        A: usize,
        B: usize,
    }
    if A <= B / K * K {
        println!("OK");
    }
    else {
        println!("NG");
    }
}