// https://atcoder.jp/contests/abc118/tasks/abc118_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    if B % A == 0 {
        println!("{}", A + B);
    }
    else {
        println!("{}", B - A);
    }
}