// https://atcoder.jp/contests/abc184/tasks/abc184_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        mut X: usize,
        S: String,
    }
    for s in S.chars() {
        if s == 'o' {
            X += 1;
        }
        else if X > 0 {
            X -= 1;
        }
    }
    println!("{}", X);
}