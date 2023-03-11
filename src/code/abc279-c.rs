// https://atcoder.jp/contests/abc279/tasks/abc279_c

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
    let mut T = Vec::new();
    for _ in 0..H {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        S.push(s);
    }
    for _ in 0..H {
        input! {
            t: String,
        }
        let t = t.chars().collect::<Vec<char>>();
        T.push(t);
    }
    let mut S_rot = vec![vec![]; W];
    let mut T_rot = vec![vec![]; W];
    for i in 0..H {
        for j in 0..W {
            S_rot[j].push(S[i][j]);
            T_rot[j].push(T[i][j]);
        }
    }
    S_rot.sort();
    T_rot.sort();
    if S_rot == T_rot {
        println!("Yes");
    }
    else {
        println!("No");
    }
}