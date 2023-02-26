// https://atcoder.jp/contests/abc063/tasks/abc063_b

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
    S.sort();
    S.dedup();
    if N == S.len() {
        println!("yes");
    }
    else {
        println!("no");
    }
}