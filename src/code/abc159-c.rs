// https://atcoder.jp/contests/abc159/tasks/abc159_c

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        L: f64,
    }
    let M = L / 3.;
    println!("{}", pow(M, 3));
}