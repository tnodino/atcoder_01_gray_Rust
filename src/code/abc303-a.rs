// https://atcoder.jp/contests/abc303/tasks/abc303_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
        T: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let T = T.chars().collect::<Vec<char>>();
    for i in 0..N {
        if S[i] == T[i] {
            continue;
        }
        if S[i] == '1' && T[i] == 'l' {
            continue
        }
        if S[i] == 'l' && T[i] == '1' {
            continue
        }
        if S[i] == '0' && T[i] == 'o' {
            continue
        }
        if S[i] == 'o' && T[i] == '0' {
            continue
        }
        println!("No");
        return;
    }
    println!("Yes");
}