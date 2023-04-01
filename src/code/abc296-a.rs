// https://atcoder.jp/contests/abc296/tasks/abc296_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    for i in 0..N-1 {
        if S[i] == S[i+1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}