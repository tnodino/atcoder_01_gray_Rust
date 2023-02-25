// https://atcoder.jp/contests/abc045/tasks/abc045_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
        h: usize,
    }
    println!("{}", (a + b) * h / 2);
}