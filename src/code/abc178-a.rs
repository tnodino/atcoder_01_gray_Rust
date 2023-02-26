// https://atcoder.jp/contests/abc178/tasks/abc178_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        x: usize,
    }
    println!("{}", x ^ 1);
}