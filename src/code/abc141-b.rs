// https://atcoder.jp/contests/abc141/tasks/abc141_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    for i in 0..S.len() {
        if i % 2 == 0 {
            if S[i] == 'L' {
                println!("No");
                return;
            }
        }
        else {
            if S[i] == 'R' {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}