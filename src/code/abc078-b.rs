// https://atcoder.jp/contests/abc078/tasks/abc078_b

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
    println!("{}", (X - Z) / (Y + Z));
}