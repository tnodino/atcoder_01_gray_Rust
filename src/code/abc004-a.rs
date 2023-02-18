// https://atcoder.jp/contests/abc004/tasks/abc004_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", N * 2);
}