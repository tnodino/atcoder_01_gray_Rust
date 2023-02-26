// https://atcoder.jp/contests/abc145/tasks/abc145_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    if N % 2 == 1 {
        println!("No");
        return;
    }
    let S = S.chars().collect::<Vec<char>>();
    let M = N / 2;
    for i in 0..M {
        if S[i] != S[i+M] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}