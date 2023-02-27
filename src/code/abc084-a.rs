// https://atcoder.jp/contests/abc084/tasks/abc084_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        M: usize,
    }
    println!("{}", 48 - M);
}