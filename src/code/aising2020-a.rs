// https://atcoder.jp/contests/aising2020/tasks/aising2020_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        L: usize,
        R: usize,
        d: usize,
    }
    println!("{}", R / d - (L - 1) / d);
}