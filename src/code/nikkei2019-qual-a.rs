// https://atcoder.jp/contests/nikkei2019-qual/tasks/nikkei2019_qual_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: isize,
        A: isize,
        B: isize,
    }
    println!("{} {}", min(A, B), max(0, (A + B) - N));
}