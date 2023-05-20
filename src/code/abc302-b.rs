// https://atcoder.jp/contests/abc302/tasks/abc302_b

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    let mut S = Vec::new();
    for _ in 0..H {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        S.push(s);
    }
    let moji = "snuke".chars().collect::<Vec<char>>();
    for i in 0..H {
        'outer1: for j in 0..=W-5 {
            for k in 0..5 {
                if S[i][j+k] != moji[k] {
                    continue 'outer1;
                }
            }
            for k in 0..5 {
                println!("{} {}", i + 1, j + 1 + k);
            }
            return;
        }
    }
    for i in 0..=H-5 {
        'outer2: for j in 0..W {
            for k in 0..5 {
                if S[i+k][j] != moji[k] {
                    continue 'outer2;
                }
            }
            for k in 0..5 {
                println!("{} {}", i + 1 + k, j + 1);
            }
            return;
        }
    }
    for i in 0..=H-5 {
        'outer3: for j in 0..=W-5 {
            for k in 0..5 {
                if S[i+k][j+k] != moji[k] {
                    continue 'outer3;
                }
            }
            for k in 0..5 {
                println!("{} {}", i + 1 + k, j + 1 + k);
            }
            return;
        }
    }
    for i in 4..H {
        'outer4: for j in 0..=W-5 {
            for k in 0..5 {
                if S[i-k][j+k] != moji[k] {
                    continue 'outer4;
                }
            }
            for k in 0..5 {
                println!("{} {}", i + 1 - k, j + 1 + k);
            }
            return;
        }
    }
    let moji = "ekuns".chars().collect::<Vec<char>>();
    for i in 0..H {
        'outer5: for j in 0..=W-5 {
            for k in 0..5 {
                if S[i][j+k] != moji[k] {
                    continue 'outer5;
                }
            }
            for k in (0..5).rev() {
                println!("{} {}", i + 1, j + 1 + k);
            }
            return;
        }
    }
    for i in 0..=H-5 {
        'outer6: for j in 0..W {
            for k in 0..5 {
                if S[i+k][j] != moji[k] {
                    continue 'outer6;
                }
            }
            for k in (0..5).rev() {
                println!("{} {}", i + 1 + k, j + 1);
            }
            return;
        }
    }
    for i in 0..=H-5 {
        'outer7: for j in 0..=W-5 {
            for k in 0..5 {
                if S[i+k][j+k] != moji[k] {
                    continue 'outer7;
                }
            }
            for k in (0..5).rev() {
                println!("{} {}", i + 1 + k, j + 1 + k);
            }
            return;
        }
    }
    for i in 4..H {
        'outer8: for j in 0..=W-5 {
            for k in 0..5 {
                if S[i-k][j+k] != moji[k] {
                    continue 'outer8;
                }
            }
            for k in (0..5).rev() {
                println!("{} {}", i + 1 - k, j + 1 + k);
            }
            return;
        }
    }
}