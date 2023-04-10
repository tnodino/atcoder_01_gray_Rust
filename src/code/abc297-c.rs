// https://atcoder.jp/contests/abc297/tasks/abc297_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    for _ in 0..H {
        input! {
            S: String,
        }
        let mut S = S.chars().collect::<Vec<char>>();
        for j in 0..W-1 {
            if S[j] == 'T' && S[j+1] == 'T' {
                S[j] = 'P';
                S[j+1] = 'C';
            }
        }
        println!("{}", S.iter().collect::<String>());
    }
}