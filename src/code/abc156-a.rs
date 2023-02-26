// https://atcoder.jp/contests/abc156/tasks/abc156_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut R: usize,
    }
    if N < 10 {
        R += 100 * (10 - N);
    }
    println!("{}", R);
}