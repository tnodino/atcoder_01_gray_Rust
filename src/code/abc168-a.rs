// https://atcoder.jp/contests/abc168/tasks/abc168_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let C = "pphbhhphph".chars().collect::<Vec<char>>();
    println!("{}on", C[N%10]);
}