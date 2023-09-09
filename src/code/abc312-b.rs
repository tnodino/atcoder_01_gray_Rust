// https://atcoder.jp/contests/abc312/tasks/abc312_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
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
            let mut black = Vec::new();
            let mut white = Vec::new();
            for x in 0..4 {
                for y in 0..4 {
                    if x == 3 || y == 3 {
                        white.push(S[i+x][j+y]);
                        white.push(S[i+8-x][j+8-y]);
                    }
                    else {
                        black.push(S[i+x][j+y]);
                        black.push(S[i+8-x][j+8-y]);
                    }
                }
            }
            if !black.contains(&'.') && !white.contains(&'#') {
                println!("{} {}", i+1, j+1);
            }
        }
    }
}