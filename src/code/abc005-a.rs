// https://atcoder.jp/contests/abc005/tasks/abc005_1

use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
    }
    println!("{}", y / x);
}