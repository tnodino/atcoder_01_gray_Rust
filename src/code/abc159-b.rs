// https://atcoder.jp/contests/abc159/tasks/abc159_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    let S = S.chars().collect::<Vec<char>>();
    let mut ans = "Yes";
    for i in 0..N/2 {
        if S[i] != S[N-i-1] {
            ans = "No";
        }
    }
    let M = (N - 1) / 2;
    for i in 0..M/2 {
        if S[i] != S[M-i-1] {
            ans = "No";
        }
    }
    let M = (N + 3) / 2;
    for i in M-1..(N+M)/2-1 {
        if S[i] != S[N-i-1] {
            ans = "No";
        }
    }
    println!("{}", ans);
}