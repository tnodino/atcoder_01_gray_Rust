// https://atcoder.jp/contests/abc186/tasks/abc186_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        W: usize,
    }
    println!("{}", N / W);
}