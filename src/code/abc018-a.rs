// https://atcoder.jp/contests/abc018/tasks/abc018_1

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
    }
    let ma = max(A, max(B, C));
    let mi = min(A, min(B, C));
    let array = [A, B, C];
    for i in 0..3 {
        println!("{}", match array[i] {
            v if v == ma => 1,
            v if v == mi => 3,
            _ => 2,
        });
    }
}