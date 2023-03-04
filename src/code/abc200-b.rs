// https://atcoder.jp/contests/abc200/tasks/abc200_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
        K: usize,
    }
    for _ in 0..K {
        if N % 200 == 0 {
            N /= 200;
        }
        else {
            N *= 1000;
            N += 200;
        }
    }
    println!("{}", N);
}