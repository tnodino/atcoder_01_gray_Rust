// https://atcoder.jp/contests/abc198/tasks/abc198_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: String,
    }
    let mut N = N.chars().collect::<Vec<char>>();
    let M = N.len();
    for _ in 0..M {
        if N == N.clone().into_iter().rev().collect::<Vec<_>>(){
            println!("Yes");
            return;
        }
        N.insert(0, '0');
    }
    println!("No");
}