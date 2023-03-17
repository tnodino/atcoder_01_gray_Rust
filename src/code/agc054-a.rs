// https://atcoder.jp/contests/agc054/tasks/agc054_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    if S[0] != S[N-1] {
        println!("1");
        return;
    }
    for i in 1..N-2 {
        if S[0] != S[i] && S[i+1] != S[N-1] {
            println!("2");
            return;
        }
    }
    println!("-1");
}