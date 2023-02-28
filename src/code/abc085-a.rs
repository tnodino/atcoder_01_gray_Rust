// https://atcoder.jp/contests/abc085/tasks/abc085_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    println!("2018{}", &S[4..]);
}