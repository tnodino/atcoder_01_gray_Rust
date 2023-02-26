// https://atcoder.jp/contests/abc066/tasks/abc066_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut N = S.len();
    N -= 2;
    loop {
        let mut flg = true;
        let M = N / 2;
        for i in 0..M {
            if S[i] != S[i+M] {
                flg = false;
                break;
            }
        }
        if flg {
            break;
        }
        N -= 2;
    }
    println!("{}", N);
}