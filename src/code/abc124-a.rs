// https://atcoder.jp/contests/abc124/tasks/abc124_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    if A > B {
        println!("{}", A + A - 1);
    }
    else if A < B {
        println!("{}", B + B - 1);
    }
    else {
        println!("{}", A + B);
    }
}