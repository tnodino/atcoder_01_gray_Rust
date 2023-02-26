// https://atcoder.jp/contests/abc206/tasks/abc206_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
    }
    N *= 108;
    N /= 100;
    if N < 206 {
        println!("Yay!");
    }
    else if N > 206 {
        println!(":(");
    }
    else {
        println!("so-so");
    }
}