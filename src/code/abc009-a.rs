// https://atcoder.jp/contests/abc009/tasks/abc009_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", (N + 1) / 2);
}