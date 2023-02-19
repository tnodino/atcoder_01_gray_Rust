// https://atcoder.jp/contests/abc011/tasks/abc011_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", N % 12 + 1);
}