// https://atcoder.jp/contests/hhkb2020/tasks/hhkb2020_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        p: [usize; N],
    }
    let mut flg = vec![false; N+1];
    let mut ans = 0;
    for i in 0..N {
        if p[i] <= N {
            flg[p[i]] = true;
        }
        while flg[ans] {
            ans += 1;
        }
        println!("{}", ans);
    }
}