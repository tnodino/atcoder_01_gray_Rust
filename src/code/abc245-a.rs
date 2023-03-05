// https://atcoder.jp/contests/abc245/tasks/abc245_a

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
    if A > C {
        println!("Aoki");
    }
    else if A < C {
        println!("Takahashi");
    }
    else {
        if B <= D {
            println!("Takahashi");
        }
        else {
            println!("Aoki");
        }
    }
}