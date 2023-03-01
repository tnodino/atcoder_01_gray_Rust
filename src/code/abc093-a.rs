// https://atcoder.jp/contests/abc093/tasks/abc093_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut S = S.chars().collect::<Vec<char>>();
    let T = "abc".chars().collect::<Vec<char>>();
    S.sort();
    if S == T {
        println!("Yes");
    }
    else {
        println!("No");
    }
}