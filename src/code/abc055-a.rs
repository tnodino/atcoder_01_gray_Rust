// https://atcoder.jp/contests/abc055/tasks/abc055_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", N * 800 - N / 15 * 200);
}