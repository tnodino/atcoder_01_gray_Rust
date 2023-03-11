// https://atcoder.jp/contests/abc282/tasks/abc282_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut S = Vec::new();
    for _ in 0..N {
        input! {
            s: String,
        }
        S.push(s.chars().collect::<Vec<char>>());
    }
    let mut ans = 0;
    for i in 0..N {
        for j in i+1..N {
            let mut flg = true;
            for k in 0..M {
                if S[i][k] == 'x' && S[j][k] == 'x' {
                    flg = false;
                }
            }
            if flg {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}