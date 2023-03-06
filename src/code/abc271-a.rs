// https://atcoder.jp/contests/abc271/tasks/abc271_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{:02X}", N);
}