// https://atcoder.jp/contests/abc098/tasks/abc098_b

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    let mut ans = 0;
    for i in 1..N {
        let X = &S[..i];
        let Y = &S[i..];
        let mut X = X.chars().collect::<Vec<char>>();
        X.sort();
        X.dedup();
        let mut cnt = 0;
        for x in X.iter() {
            if Y.contains(*x) {
                cnt += 1;
            }
        }
        ans = max(ans, cnt);
    }
    println!("{}", ans);
}