// https://atcoder.jp/contests/abc258/tasks/abc258_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
    }
    println!("{:02}:{:02}", 21 + K / 60, K % 60);
}