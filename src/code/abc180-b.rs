// https://atcoder.jp/contests/abc180/tasks/abc180_b

use proconio::input;
use proconio::fastout;
use std::cmp::max;
use num::pow;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        x: [isize; N],
    }
    let x = x.iter().map(|&x| x.abs()).collect::<Vec<_>>();
    let mut man = 0;
    let mut euc = 0;
    let mut che = 0;
    for v in x {
        man += v;
        euc += pow(v, 2);
        che = max(che, v);
    }
    println!("{}\n{}\n{}", man, sqrt(euc as f64), che);
}