// https://atcoder.jp/contests/abc303/tasks/abc303_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut flg = vec![vec![true; N]; N];
    for _ in 0..M {
        input! {
            a: [usize; N],
        }
        for i in 0..N-1 {
            flg[a[i]-1][a[i+1]-1] = false;
            flg[a[i+1]-1][a[i]-1] = false;
        }
    }
    let mut ans = 0;
    for i in 0..N {
        for j in i+1..N {
            if flg[i][j] {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}