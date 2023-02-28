// https://atcoder.jp/contests/apc001/tasks/apc001_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        Y: usize,
    }
    if X % Y == 0 {
        println!("-1");
    }
    else {
        println!("{}", X);
    }
}