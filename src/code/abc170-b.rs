// https://atcoder.jp/contests/abc170/tasks/abc170_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        Y: usize,
    }
    for a in 0..=Y/4 {
        let b = (Y - a * 4) / 2;
        if a + b == X && a * 4 + b * 2 == Y {
            println!("Yes");
            return;
        }
    }
    println!("No");
}