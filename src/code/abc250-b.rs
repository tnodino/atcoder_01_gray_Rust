// https://atcoder.jp/contests/abc250/tasks/abc250_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
    }
    for i in 0..N*A {
        for j in 0..N*B {
            if (i % (A * 2) < A) == (j % (B * 2) < B) {
                print!(".");
            }
            else {
                print!("#");
            }
        }
        println!();
    }
}