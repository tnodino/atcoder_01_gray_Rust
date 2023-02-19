// https://atcoder.jp/contests/abc013/tasks/abc013_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: char,
    }
    println!("{}", X as usize - 64);
}