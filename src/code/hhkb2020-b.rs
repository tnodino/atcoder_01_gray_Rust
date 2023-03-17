// https://atcoder.jp/contests/hhkb2020/tasks/hhkb2020_b

use proconio::input;
use proconio::fastout;

#[fastout]
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
    let mut ans = 0;
    for i in 0..H {
        for j in 0..W {
            if S[i][j] == '.' {
                if i + 1 < H && S[i+1][j] == '.' {
                    ans += 1;
                }
                if j + 1 < W && S[i][j+1] == '.' {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}