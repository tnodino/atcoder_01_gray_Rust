// https://atcoder.jp/contests/abc266/tasks/abc266_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    println!("{}", S.chars().nth(N/2).unwrap());
}