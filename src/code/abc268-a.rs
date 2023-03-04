// https://atcoder.jp/contests/abc268/tasks/abc268_a

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
    }
    let mut vec = vec![A, B, C, D, E];
    vec.sort();
    vec.dedup();
    println!("{}", vec.len());
}