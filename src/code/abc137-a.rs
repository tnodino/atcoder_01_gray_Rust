// https://atcoder.jp/contests/abc137/tasks/abc137_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: isize,
        B: isize,
    }
    println!("{}", [A + B, A - B, A * B].iter().max().unwrap());
}