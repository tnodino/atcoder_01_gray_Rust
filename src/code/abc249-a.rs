// https://atcoder.jp/contests/abc249/tasks/abc249_a

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
        E: usize,
        F: usize,
        X: usize,
    }
    let mut Ta = X / (A + C) * A * B;
    if X % (A + C) <= A {
        Ta += X % (A + C) * B;
    }
    else {
        Ta += A * B;
    }
    let mut Ao = X / (D + F) * D * E;
    if X % (D + F) <= D {
        Ao += X % (D + F) * E;
    }
    else {
        Ao += D * E;
    }
    if Ta > Ao {
        println!("Takahashi");
    }
    else if Ta < Ao {
        println!("Aoki");
    }
    else {
        println!("Draw");
    }
}