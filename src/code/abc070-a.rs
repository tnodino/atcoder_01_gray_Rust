// https://atcoder.jp/contests/abc070/tasks/abc070_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: String,
    }
    if N == N.chars().rev().collect::<String>() {
        println!("Yes");
    }
    else {
        println!("No");
    }
}