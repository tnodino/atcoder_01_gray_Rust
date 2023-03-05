// https://atcoder.jp/contests/abc129/tasks/abc129_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        P: usize,
        Q: usize,
        R: usize,
    }
    println!("{}", [P + Q, P + R, Q + R].iter().min().unwrap());
}