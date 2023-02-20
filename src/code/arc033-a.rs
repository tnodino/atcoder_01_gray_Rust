// https://atcoder.jp/contests/arc033/tasks/arc033_1

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