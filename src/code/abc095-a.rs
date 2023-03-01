// https://atcoder.jp/contests/abc095/tasks/abc095_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    println!("{}", 700 + S.chars().filter(|&x| x == 'o').count() * 100);
}