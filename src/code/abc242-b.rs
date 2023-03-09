// https://atcoder.jp/contests/abc242/tasks/abc242_b

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
    println!("{}", S.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(""))
}