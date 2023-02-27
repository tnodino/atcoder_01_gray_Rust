// https://atcoder.jp/contests/abc083/tasks/abc083_a

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
    let L = A + B;
    let R = C + D;
    if L > R {
        println!("Left");
    }
    else if L < R {
        println!("Right");
    }
    else {
        println!("Balanced");
    }
}