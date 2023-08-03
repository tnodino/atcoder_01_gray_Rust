// https://atcoder.jp/contests/abc311/tasks/abc311_b

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        D: usize,
    }
    let mut S = Vec::new();
    for _ in 0..N {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        S.push(s);
    }
    let mut cnt = 0;
    let mut ans = 0;
    for j in 0..D {
        let mut flg = true;
        for i in 0..N {
            if S[i][j] == 'x' {
                flg = false;
            }
        }
        if flg {
            cnt += 1;
        }
        else {
            cnt = 0;
        }
        ans = max(ans, cnt);
    }
    println!("{}", ans);
}