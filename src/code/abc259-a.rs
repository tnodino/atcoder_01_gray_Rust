// https://atcoder.jp/contests/abc259/tasks/abc259_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        M: usize,
        X: usize,
        T: usize,
        D: usize,
    }
    if M >= X {
        println!("{}", T);
    }
    else {
        println!("{}", T - (X - M) * D);
    }
}