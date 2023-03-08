// https://atcoder.jp/contests/abc164/tasks/abc164_b

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
    if (A + D - 1) / D >= (C + B - 1) / B {
        println!("Yes");
    }
    else {
        println!("No");
    }
}