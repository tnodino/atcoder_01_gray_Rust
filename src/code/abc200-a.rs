// https://atcoder.jp/contests/abc200/tasks/abc200_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", (N + 99) / 100);
}