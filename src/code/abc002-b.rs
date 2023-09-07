// https://atcoder.jp/contests/abc002/tasks/abc002_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut W: String,
    }
    let C = "aiueo".chars().collect::<Vec<char>>();
    for c in C.iter() {
        W = W.replace(*c, "");
    }
    println!("{}", W);
}