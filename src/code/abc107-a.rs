// https://atcoder.jp/contests/abc107/tasks/abc107_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        i: usize,
    }
    println!("{}", N - i + 1);
}