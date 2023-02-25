// https://atcoder.jp/contests/abc051/tasks/abc051_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String,
    }
    println!("{}", s.replace(",", " "));
}