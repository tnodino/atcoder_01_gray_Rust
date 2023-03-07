// https://atcoder.jp/contests/abc139/tasks/abc139_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    if B == 1 {
        println!("0");
        return;
    }
    for i in 1..=B {
        if (A - 1) * i + 1 >= B {
            println!("{}", i);
            return;
        }
    }
}