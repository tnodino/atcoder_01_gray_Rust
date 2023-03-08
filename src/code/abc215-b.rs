// https://atcoder.jp/contests/abc215/tasks/abc215_b

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut k = 0;
    while pow(2, k + 1) <= N {
        k += 1;
    }
    println!("{}", k);
}