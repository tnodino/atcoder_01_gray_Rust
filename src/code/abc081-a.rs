// https://atcoder.jp/contests/abc081/tasks/abc081_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String,
    }
    println!("{}", s.chars().filter(|&x| x == '1').count())
}