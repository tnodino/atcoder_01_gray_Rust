// https://atcoder.jp/contests/abc131/tasks/abc131_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    for i in 0..3 {
        if S[i] == S[i+1] {
            println!("Bad");
            return;
        }
    }
    println!("Good");
}