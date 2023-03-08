// https://atcoder.jp/contests/abc199/tasks/abc199_b

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [isize; N],
        B: [isize; N],
    }
    println!("{}", max(0, B.iter().min().unwrap() -  A.iter().max().unwrap() + 1));
}