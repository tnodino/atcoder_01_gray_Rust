// https://atcoder.jp/contests/abc158/tasks/abc158_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    if !(S.contains("A")) || !(S.contains("B")) {
        println!("No");
    }
    else {
        println!("Yes");
    }
}