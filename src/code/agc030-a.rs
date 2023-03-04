// https://atcoder.jp/contests/agc030/tasks/agc030_a

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
    if A + B + 1 >= C {
        println!("{}", B + C);
    }
    else {
        println!("{}", B + (A + B + 1));
    }
}