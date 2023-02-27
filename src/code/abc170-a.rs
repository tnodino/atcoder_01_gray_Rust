// https://atcoder.jp/contests/abc170/tasks/abc170_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        x1: usize,
        x2: usize,
        x3: usize,
        x4: usize,
        x5: usize,
    }
    let x = [0, x1, x2, x3, x4, x5];
    for i in 1..=5 {
        if x[i] != i {
            println!("{}", i);
            return;
        }
    }
}