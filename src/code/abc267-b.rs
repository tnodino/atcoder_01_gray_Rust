// https://atcoder.jp/contests/abc267/tasks/abc267_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = 7;
    let S = S.chars().collect::<Vec<char>>();
    if S[0] == '1' {
        println!("No");
        return;
    }
    let row = vec![vec![6],
        vec![3],
        vec![1, 7],
        vec![0, 4],
        vec![2, 8],
        vec![5],
        vec![9]];
    let mut flg = vec![false; N];
    for i in 0..N {
        for j in 0..row[i].len() {
            if S[row[i][j]] == '1' {
                flg[i] = true;
            }
        }
    }
    for l in 0..N {
        for r in l+2..N {
            if !flg[l] || !flg[r] {
                continue;
            }
            if !flg[l+1] {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}