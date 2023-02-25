// https://atcoder.jp/contests/abc025/tasks/abc025_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        N: usize,
    }
    let S: Vec<char> = S.chars().collect();
    println!("{}{}", S[(N-1)/5], S[(N-1)%5]);
}