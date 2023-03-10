// https://atcoder.jp/contests/abl/tasks/abl_b

use proconio::input;
use proconio::fastout;
use std::cmp::max;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
        D: usize,
    }
    if max(A, C) <= min(B, D) {
        println!("Yes");
    }
    else {
        println!("No");
    }
}