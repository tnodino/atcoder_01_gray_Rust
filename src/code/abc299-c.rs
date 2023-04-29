// https://atcoder.jp/contests/abc299/tasks/abc299_c

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
    let S = S.chars().collect::<Vec<char>>();
    let mut ans = 0;
    let mut cnt = (N as isize) * -1;
    for i in 0..N {
        if S[i] == 'o' {
            cnt += 1;
        }
        else {
            ans = max(ans, cnt);
            cnt = 0;
        }
    }
    ans = max(ans, cnt);
    let mut cnt = (N as isize) * -1;
    for i in (0..N).rev() {
        if S[i] == 'o' {
            cnt += 1;
        }
        else {
            ans = max(ans, cnt);
            cnt = 0;
        }
    }
    ans = max(ans, cnt);
    if ans == 0 {
        println!("-1");
    }
    else {
        println!("{}", ans);
    }
}