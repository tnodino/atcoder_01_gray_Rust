// https://atcoder.jp/contests/jsc2021/tasks/jsc2021_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        Y: usize,
        Z: usize,
    }
    println!("{}", (Y * Z - 1) / X);
}