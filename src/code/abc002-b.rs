// https://atcoder.jp/contests/abc002/tasks/abc002_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut W: String,
    }
    let vowel = "aiueo".to_string();
    for v in vowel.chars() {
        W = W.replace(v, "");
    }
    println!("{}", W);
}