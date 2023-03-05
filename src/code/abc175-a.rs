// https://atcoder.jp/contests/abc175/tasks/abc175_a

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
    let mut cnt = 0;
    for i in 0..3 {
        if S[i] == 'R' {
            cnt += 1;
        }
        else {
            cnt = 0;
        }
        ans = max(ans, cnt);
    }
    println!("{}", max(ans, cnt));
}