// https://atcoder.jp/contests/abc043/tasks/abc043_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", (N + 1) * N / 2);
}