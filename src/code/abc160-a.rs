// https://atcoder.jp/contests/abc160/tasks/abc160_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    if S[2] == S[3] && S[4] == S[5] {
        println!("Yes");
    }
    else {
        println!("No");
    }
}