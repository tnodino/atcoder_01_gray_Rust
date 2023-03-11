// https://atcoder.jp/contests/abc177/tasks/abc177_b

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        T: String,
    }
    let N = S.len();
    let M = T.len();
    let S = S.chars().collect::<Vec<char>>();
    let T = T.chars().collect::<Vec<char>>();
    let mut ans = M;
    for x in 0..=N-M {
        let mut cnt = 0;
        for i in 0..M {
            if S[i+x] != T[i] {
                cnt += 1;
            }
        }
        ans = min(ans, cnt);
    }
    println!("{}", ans);
}