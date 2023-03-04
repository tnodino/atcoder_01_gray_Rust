// https://atcoder.jp/contests/abc125/tasks/abc125_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        T: usize,
    }
    println!("{}", T / A * B);
}