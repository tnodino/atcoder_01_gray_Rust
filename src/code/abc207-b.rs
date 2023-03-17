// https://atcoder.jp/contests/abc207/tasks/abc207_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
        D: usize,
    }
    let mut red = 0;
    let mut blue = A;
    for i in 1..=A {
        blue += B;
        red += C;
        if blue <= red * D {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}