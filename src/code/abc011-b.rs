// https://atcoder.jp/contests/abc011/tasks/abc011_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut S: String,
    }
    S = S.to_lowercase();
    let F = S.chars().nth(0).unwrap().to_uppercase();
    println!("{}{}", F, &S[1..]);
}