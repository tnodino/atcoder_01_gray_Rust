// https://atcoder.jp/contests/abc077/tasks/abc077_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        C: [String; 2],
    }
    let S: Vec<Vec<char>> = C.iter().map(|x| x.chars().collect::<Vec<char>>()).collect();
    let mut T = vec![vec!['?'; 3]; 2];
    for i in 0..2 {
        for j in 0..3 {
            T[i][j] = S[1-i][2-j];
        }
    }
    if S == T {
        println!("YES");
    }
    else {
        println!("NO");
    }
}