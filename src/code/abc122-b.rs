// https://atcoder.jp/contests/abc122/tasks/abc122_b

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut ans = 0;
    for i in 0..S.len() {
        for j in i..S.len() {
            if !"ACGT".contains(S[j]) {
                ans = max(ans, j - i);
                break;
            }
            if j == S.len() - 1 {
                ans = max(ans, j - i + 1);
            }
        }
    }
    println!("{}", ans);
}