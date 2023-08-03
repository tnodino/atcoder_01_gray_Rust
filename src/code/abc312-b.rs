// https://atcoder.jp/contests/abc312/tasks/abc312_b

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
        let s = s.chars().collect::<Vec<char>>();
        S.push(s);
    }
    for i in 0..=N-9 {
        for j in 0..=M-9 {
            let mut flg = true;
            for k in 0..4 {
                for l in 0..4 {
                    if k == 3 || l == 3 {
                        if S[i+k][j+l] != '.' {
                            flg = false;
                        }
                    }
                    else {
                        if S[i+k][j+l] != '#' {
                            flg = false;
                        }
                    }
                    if k == 0 || l == 0 {
                        if S[i+5+k][j+5+l] != '.' {
                            flg = false;
                        }
                    }
                    else {
                        if S[i+5+k][j+5+l] != '#' {
                            flg = false;
                        }
                    }
                }
            }
            if flg {
                println!("{} {}", i+1, j+1);
            }
        }
    }
}