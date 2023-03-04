// https://atcoder.jp/contests/abc220/tasks/abc220_a

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
    for i in A..=B {
        if i % C == 0 {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}