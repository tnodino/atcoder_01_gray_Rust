// https://atcoder.jp/contests/abc233/tasks/abc233_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        L: usize,
        R: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    print!("{}", S[..L-1].iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(""));
    print!("{}", S[L-1..R].iter().rev().map(|&x| x.to_string()).collect::<Vec<String>>().join(""));
    print!("{}", S[R..].iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(""));
    println!();
}