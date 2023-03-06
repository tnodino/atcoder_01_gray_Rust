// https://atcoder.jp/contests/abc139/tasks/abc139_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", (N - 1) * N / 2);
}