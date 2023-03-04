// https://atcoder.jp/contests/abc113/tasks/abc113_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        Y: usize,
    }
    println!("{}", X + Y / 2);
}
