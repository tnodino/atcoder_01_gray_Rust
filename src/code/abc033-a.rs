// https://atcoder.jp/contests/abc033/tasks/abc033_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: String,
    }
    let mut N: Vec<char> = N.chars().collect();
    N.dedup();
    if N.len() == 1 {
        println!("SAME");
    }
    else {
        println!("DIFFERENT");
    }
}