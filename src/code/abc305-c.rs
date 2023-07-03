// https://atcoder.jp/contests/abc305/tasks/abc305_c

use proconio::input;
use proconio::fastout;
use std::cmp::{min, max};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    let mut a = H;
    let mut b = W;
    let mut c = 0;
    let mut d = 0;
    let mut S = Vec::new();
    for i in 0..H {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        for j in 0..W {
            if s[j] == '#' {
                a = min(a, i);
                b = min(b, j);
                c = max(c, i);
                d = max(d, j);
            }
        }
        S.push(s);
    }
    for i in a..=c {
        for j in b..=d {
            if S[i][j] == '.' {
                println!("{} {}", i+1, j+1);
                return;
            }
        }
    }
}