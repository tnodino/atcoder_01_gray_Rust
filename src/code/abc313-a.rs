// https://atcoder.jp/contests/abc313/tasks/abc313_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: [usize; N],
    }
    let mut ma = 0;
    for i in 1..N {
        ma = max(ma, P[i]);
    }
    if P[0] > ma {
        println!("0");
    }
    else {
        println!("{}", ma - P[0] + 1);
    }
}