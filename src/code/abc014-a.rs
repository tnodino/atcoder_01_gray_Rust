// https://atcoder.jp/contests/abc014/tasks/abc014_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    println!("{}", (b - a % b) % b);
}