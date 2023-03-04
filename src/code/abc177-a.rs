// https://atcoder.jp/contests/abc177/tasks/abc177_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        D: usize,
        T: usize,
        S: usize,
    }
    if (D + S - 1) / S <= T {
        println!("Yes");
    }
    else {
        println!("No");
    }
}