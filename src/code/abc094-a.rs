// https://atcoder.jp/contests/abc094/tasks/abc094_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        X: usize,
    }
    if A <= X && X <= A + B {
        println!("YES");
    }
    else {
        println!("NO");
    }
}