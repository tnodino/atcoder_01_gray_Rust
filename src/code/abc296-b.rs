// https://atcoder.jp/contests/abc296/tasks/abc296_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let mut S = Vec::new();
    for _ in 0..8 {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        S.push(s);
    }
    for i in 0..8 {
        for j in 0..8 {
            if S[i][j] == '*' {
                println!("{}{}", (j as u8 + 97) as char, 8 - i);
            }
        }
    }
}