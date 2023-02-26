// https://atcoder.jp/contests/abc065/tasks/abc065_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        A: usize,
        B: usize,
    }
    if B <= A {
        println!("delicious");
    }
    else if B <= A + X {
        println!("safe");
    }
    else {
        println!("dangerous");
    }
}