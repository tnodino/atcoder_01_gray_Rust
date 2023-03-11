// https://atcoder.jp/contests/abc221/tasks/abc221_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        T: String,
    }
    if S == T {
        println!("Yes");
        return;
    }
    let mut S = S.chars().collect::<Vec<char>>();
    let T = T.chars().collect::<Vec<char>>();
    for i in 0..S.len()-1 {
        if S[i] != T[i] {
            S.swap(i, i + 1);
            if S == T {
                println!("Yes");
                return;
            }
            S.swap(i, i + 1);
        }
    }
    println!("No");
}