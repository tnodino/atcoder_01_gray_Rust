// https://atcoder.jp/contests/abc235/tasks/abc235_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut X: usize,
    }
    let mut S = 0;
    for _ in 0..3 {
        S += X % 10;
        X /= 10;
    }
    println!("{}", S + S * 10 + S * 100);
}