// https://atcoder.jp/contests/arc144/tasks/arc144_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
    }
    let M = N * 2;
    let mut x = Vec::new();
    if N % 4 > 0 {
        x.push(N % 4);
    }
    for _ in 0..N/4 {
        x.push(4);
    }
    println!("{}", M);
    println!("{}", x.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(""));
}