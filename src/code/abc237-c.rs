// https://atcoder.jp/contests/abc237/tasks/abc237_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut N = S.len();
    let mut S = S.chars().collect::<Vec<char>>();
    S.reverse();
    let mut idx = 0;
    while idx < N / 2 {
        if S[idx] != S[N-idx-1] {
            if S[idx] == 'a' {
                N += 1;
                S.push('a');
            }
            else {
                println!("No");
                return;
            }
        }
        else {
            idx += 1;
        }
    }
    println!("Yes");
}