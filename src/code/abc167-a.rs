// https://atcoder.jp/contests/abc167/tasks/abc167_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        T: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let T = T.chars().collect::<Vec<char>>();
    for i in 0..S.len() {
        if S[i] != T[i] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}