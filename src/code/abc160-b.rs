// https://atcoder.jp/contests/abc160/tasks/abc160_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
    }
    println!("{}", X / 500 * 1000 + X % 500 / 5 * 5);
}