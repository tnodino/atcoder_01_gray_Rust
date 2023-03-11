// https://atcoder.jp/contests/abc253/tasks/abc253_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    let mut S = Vec::new();
    for _ in 0..H {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        S.push(s);
    }
    let mut token = Vec::new();
    for i in 0..H {
        for j in 0..W {
            if S[i][j] == 'o' {
                token.push(i as isize);
                token.push(j as isize);
            }
        }
    }
    println!("{}", (token[2] - token[0]).abs() + (token[3] - token[1]).abs());
}