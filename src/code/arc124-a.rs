// https://atcoder.jp/contests/abc214/tasks/abc214_c

use proconio::input;
use proconio::fastout;

const MOD: usize = 998_244_353;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut flg = vec![true; N];
    let mut cnt = vec![0; N];
    for _ in 0..K {
        input! {
            c: char,
            k: usize,
        }
        flg[k-1] = false;
        if c == 'L' {
            for i in k..N {
                cnt[i] += 1;
            }
        }
        else {
            for i in 0..k-1 {
                cnt[i] += 1;
            }
        }
    }
    let mut ans = 1;
    for i in 0..N {
        if flg[i] {
            ans *= cnt[i];
            ans %= MOD;
        }
    }
    println!("{}", ans);
}