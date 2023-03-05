// https://atcoder.jp/contests/abc292/tasks/abc292_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }
    let mut cnt = vec![0; N+1];
    let mut flg = vec![false; N+1];
    for _ in 0..Q {
        input! {
            c: usize,
            x: usize,
        }
        if c == 1 {
            cnt[x] += 1;
            if cnt[x] == 2 {
                flg[x] = true;
            }
        }
        else if c == 2 {
            flg[x] = true;
        }
        else {
            if flg[x] {
                println!("Yes");
            }
            else {
                println!("No");
            }
        }
    }
}