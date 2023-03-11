// https://atcoder.jp/contests/abc212/tasks/abc212_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut x: usize,
    }
    let mut X = Vec::new();
    for _ in 0..4 {
        X.push(x % 10);
        x /= 10;
    }
    let X = X.into_iter().rev().collect::<Vec<_>>();
    if X[0] == X[1] && X[1] == X[2] && X[2] == X[3] {
        println!("Weak");
        return;
    }
    for i in 0..3 {
        if (X[i] + 1) % 10 != X[i+1] {
            println!("Strong");
            return;
        }
    }
    println!("Weak");
}