// https://atcoder.jp/contests/abc190/tasks/abc190_a

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
    if A > B {
        println!("Takahashi");
    }
    else if A < B {
        println!("Aoki")
    }
    else {
        if C == 0 {
            println!("Aoki");
        }
        else {
            println!("Takahashi");
        }
    }
}