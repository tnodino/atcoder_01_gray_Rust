// https://atcoder.jp/contests/abc011/tasks/abc011_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    let mut S = S.chars().collect::<Vec<char>>();
    S[0] = S[0].to_uppercase().next().unwrap();
    for i in 1..N {
        S[i] = S[i].to_lowercase().next().unwrap();
    }
    println!("{}", S.iter().collect::<String>());
}