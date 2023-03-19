// https://atcoder.jp/contests/abc262/tasks/abc262_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut flg = vec![vec![false; N]; N];
    for _ in 0..M {
        input! {
            U: usize,
            V: usize,
        }
        flg[U-1][V-1] = true;
        flg[V-1][U-1] = true;
    }
    let mut ans = 0;
    for a in 0..N {
        for b in a+1..N {
            for c in b+1..N {
                if flg[a][b] && flg[a][c] && flg[b][c] {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}