// https://atcoder.jp/contests/abc005/tasks/abc005_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (x, y): (usize, usize),
    }
    println!("{}", y / x);
}