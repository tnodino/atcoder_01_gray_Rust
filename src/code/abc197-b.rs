// https://atcoder.jp/contests/abc197/tasks/abc197_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        X: usize,
        Y: usize,
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
    let mut x = X - 1;
    while x < H && S[x][Y-1] == '.' {
        ans += 1;
        x = x.wrapping_add(!0usize);
    }
    let mut x = X - 1;
    while x < H && S[x][Y-1] == '.' {
        ans += 1;
        x = x.wrapping_add(1);
    }
    let mut y = Y - 1;
    while y < W && S[X-1][y] == '.' {
        ans += 1;
        y = y.wrapping_add(!0usize);
    }
    let mut y = Y - 1;
    while y < W && S[X-1][y] == '.' {
        ans += 1;
        y = y.wrapping_add(1);
    }
    println!("{}", ans - 3);
}