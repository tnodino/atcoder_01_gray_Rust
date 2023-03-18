// https://atcoder.jp/contests/arc120/tasks/arc120_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }
    let mut su1 = 0;
    let mut su2 = 0;
    let mut ma = 0;
    for i in 0..N {
        su1 += A[i];
        su2 += su1;
        ma = max(ma, A[i]);
        println!("{}", (i + 1) * ma + su2);
    }
}