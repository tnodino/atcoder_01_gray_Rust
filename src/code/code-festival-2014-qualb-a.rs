// https://atcoder.jp/contests/code-festival-2014-qualb/tasks/code_festival_qualB_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    println!("{}", max(A, B));
}