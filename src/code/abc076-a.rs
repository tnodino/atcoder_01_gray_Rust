// https://atcoder.jp/contests/abc076/tasks/abc076_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        R: isize,
        G: isize,
    }
    println!("{}", G * 2 - R);
}