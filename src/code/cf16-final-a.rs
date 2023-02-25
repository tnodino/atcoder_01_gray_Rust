// https://atcoder.jp/contests/cf16-final/tasks/codefestival_2016_final_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        S: [[String; W]; H],
    }
    for i in 0..H {
        for j in 0..W {
            if S[i][j] == "snuke" {
                println!("{}{}", (j as u8 + 65) as char, i + 1);
            }
        }
    }
}