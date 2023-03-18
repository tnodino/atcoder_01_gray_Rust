// https://atcoder.jp/contests/abc267/tasks/abc267_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    if S[0] == '1' {
        println!("No");
        return;
    }
    let mut flg = [false; 7];
    let idx = [3, 2, 4, 1, 3, 5, 0, 2, 4, 6];
    for i in 0..10 {
        if S[i] == '1' {
            flg[idx[i]] = true;
        }
    }
    for i in 0..6 {
        if flg[i] && !flg[i+1] {
            for j in i+2..7 {
                if flg[j] {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}