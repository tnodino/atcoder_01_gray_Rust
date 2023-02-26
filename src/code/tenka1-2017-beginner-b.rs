// https://atcoder.jp/contests/tenka1-2017-beginner/tasks/tenka1_2017_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut M = 0;
    let mut point = 0;
    for _ in 0..N {
        input! {
            A: usize,
            B: usize,
        }
        if A > M {
            M = A;
            point = B;
        }
    }
    println!("{}", M + point);
}