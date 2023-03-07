// https://atcoder.jp/contests/abc144/tasks/abc144_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    for i in 1..=9 {
        for j in 1..=9 {
            if i * j == N {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}