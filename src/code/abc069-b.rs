// https://atcoder.jp/contests/abc069/tasks/abc069_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String,
    }
    let s = s.chars().collect::<Vec<char>>();
    println!("{}{}{}", s[0], s.len() - 2, s[s.len()-1]);
}