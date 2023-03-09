// https://atcoder.jp/contests/abc157/tasks/abc157_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: [[usize; 3]; 3],
        N: usize,
    }
    let mut flg = [[false; 3]; 3];
    for _ in 0..N {
        input! {
            b: usize,
        }
        for i in 0..3 {
            for j in 0..3 {
                if A[i][j] == b {
                    flg[i][j] = true;
                }
            }
        }
    }
    let mut ans = "No";
    for i in 0..3 {
        if flg[i][0] && flg[i][1] && flg[i][2] {
            ans = "Yes";
        }
        if flg[0][i] && flg[1][i] && flg[2][i] {
            ans = "Yes";
        }
    }
    if flg[0][0] && flg[1][1] && flg[2][2] {
        ans = "Yes";
    }
    if flg[0][2] && flg[1][1] && flg[2][0] {
        ans = "Yes";
    }
    println!("{}", ans);
}