// https://atcoder.jp/contests/arc052/tasks/arc052_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    for s in S.chars() {
        if '0' <= s && s <= '9' {
            print!("{}", s);
        }
    }
    println!("");
}