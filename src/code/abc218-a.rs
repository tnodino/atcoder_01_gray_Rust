// https://atcoder.jp/contests/abc218/tasks/abc218_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    if S.chars().nth(N-1).unwrap() == 'o' {
        println!("Yes");
    }
    else {
        println!("No");
    }
}