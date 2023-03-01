// https://atcoder.jp/contests/abc225/tasks/abc225_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut S = S.chars().collect::<Vec<char>>();
    S.sort();
    S.dedup();
    println!("{}", (S.len() + 1) * S.len() / 2);
}