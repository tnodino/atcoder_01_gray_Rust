// https://atcoder.jp/contests/tenka1-2019-beginner/tasks/tenka1_2019_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
        K: usize,
    }
    let mut S = S.chars().collect::<Vec<char>>();
    for i in 0..N {
        if S[i] != S[K-1] {
            S[i] = '*';
        }
    }
    println!("{}", S.iter().map(|x| x.to_string()).collect::<String>());
}