// https://atcoder.jp/contests/abc082/tasks/abc082_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    println!("{}", (a + b + 1) / 2);
}