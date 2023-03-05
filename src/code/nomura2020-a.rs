// https://atcoder.jp/contests/nomura2020/tasks/nomura2020_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H1: usize,
        M1: usize,
        H2: usize,
        M2: usize,
        K: usize,
    }
    let T1 = H1 * 60 + M1;
    let T2 = H2 * 60 + M2;
    println!("{}", T2 - T1 - K);
}