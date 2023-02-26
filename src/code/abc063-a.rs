// https://atcoder.jp/contests/abc063/tasks/abc063_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    if A + B >= 10 {
        println!("error");
    }
    else {
        println!("{}", A + B);
    }
}