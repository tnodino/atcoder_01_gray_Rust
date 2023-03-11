// https://atcoder.jp/contests/abc273/tasks/abc273_b

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut X: usize,
        K: usize,
    }
    for _ in 0..K {
        let x = X % 10;
        X /= 10;
        if x >= 5 {
            X += 1;
        }
    }
    println!("{}", X * pow(10, K));
}