// https://atcoder.jp/contests/abc148/tasks/abc148_b

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
        print!("{}{}", S[i], T[i]);
    }
    println!();
}