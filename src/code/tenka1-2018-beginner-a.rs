// https://atcoder.jp/contests/tenka1-2018-beginner/tasks/tenka1_2018_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut S: String,
    }
    if S.len() == 3 {
        S = S.chars().rev().collect::<String>();
    }
    println!("{}", S);
}