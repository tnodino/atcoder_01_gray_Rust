// https://atcoder.jp/contests/abc153/tasks/abc153_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        A: usize,
    }
    println!("{}", (H + A - 1) / A);
}