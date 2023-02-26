// https://atcoder.jp/contests/abc230/tasks/abc230_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
    }
    if N >= 42 {
        N += 1;
    }
    println!("AGC{:03}", N);
}