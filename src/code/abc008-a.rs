// https://atcoder.jp/contests/abc008/tasks/abc008_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (S, T): (usize, usize),
    }
    println!("{}", T - S + 1);
}