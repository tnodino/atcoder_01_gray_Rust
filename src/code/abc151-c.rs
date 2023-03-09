// https://atcoder.jp/contests/abc151/tasks/abc151_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut cnt = vec![0; N+1];
    let mut flg = vec![true; N+1];
    let mut ac = 0;
    let mut wa = 0;
    for _ in 0..M {
        input! {
            p: usize,
            S: String,
        }
        if S == "WA" {
            cnt[p] += 1;
        }
        else if flg[p] {
            ac += 1;
            wa += cnt[p];
            flg[p] = false;
        }
    }
    println!("{} {}", ac, wa);
}