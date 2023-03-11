// https://atcoder.jp/contests/agc046/tasks/agc046_a

use proconio::input;
use proconio::fastout;
use num::integer::lcm;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
    }
    println!("{}", lcm(X, 360) / X);
}