// https://atcoder.jp/contests/abc134/tasks/abc134_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        D: usize,
    }
    let M = D * 2 + 1;
    println!("{}", (N + M - 1) / M);
}