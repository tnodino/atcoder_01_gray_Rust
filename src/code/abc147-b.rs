// https://atcoder.jp/contests/abc147/tasks/abc147_b

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
    let mut ans = 0;
    for i in 0..N/2 {
        if S[i] != S[N-i-1] {
            ans += 1;
        }
    }
    println!("{}", ans);
}