// https://atcoder.jp/contests/abc186/tasks/abc186_b

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    let mut sum = 0;
    let mut mi = 999;
    for _ in 0..H {
        input! {
            A: [usize; W],
        }
        sum += A.iter().sum::<usize>();
        mi = min(mi, *A.iter().min().unwrap());
    }
    println!("{}", sum - mi * H * W);
}