// https://atcoder.jp/contests/abc205/tasks/abc205_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: f64,
        B: f64,
    }
    println!("{}", A / 100. * B);
}