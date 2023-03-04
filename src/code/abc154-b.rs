// https://atcoder.jp/contests/abc154/tasks/abc154_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    println!("{}", "x".repeat(S.len()));
}