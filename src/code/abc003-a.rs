// https://atcoder.jp/contests/abc003/tasks/abc003_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = 10000;
    println!("{}", (N * M + M) / 2);
}