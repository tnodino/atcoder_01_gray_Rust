// https://atcoder.jp/contests/abc260/tasks/abc260_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    if S[0] != S[1] && S[0] != S[2] {
        println!("{}", S[0]);
    }
    else if S[1] != S[0] && S[1] != S[2] {
        println!("{}", S[1]);
    }
    else if S[2] != S[0] && S[2] != S[1] {
        println!("{}", S[2]);
    }
    else {
        println!("-1");
    }
}