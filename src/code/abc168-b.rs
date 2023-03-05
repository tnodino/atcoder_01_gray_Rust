// https://atcoder.jp/contests/abc168/tasks/abc168_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
        S: String,
    }
    if S.len() <= K {
        println!("{}", S);
    }
    else {
        println!("{}...", &S[..K]);
    }
}