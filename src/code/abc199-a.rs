// https://atcoder.jp/contests/abc199/tasks/abc199_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
    }
    if A * A + B * B < C * C {
        println!("Yes");
    }
    else {
        println!("No");
    }
}