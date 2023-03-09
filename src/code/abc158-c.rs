// https://atcoder.jp/contests/abc158/tasks/abc158_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    for x in 1..=1010 {
        if x * 8 / 100 == A && x * 10 / 100 == B {
            println!("{}", x);
            return;
        }
    }
    println!("-1");
}