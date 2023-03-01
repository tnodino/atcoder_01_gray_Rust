// https://atcoder.jp/contests/abc232/tasks/abc232_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    println!("{}", (S[0] as usize - 48) * (S[2] as usize - 48));
}