// https://atcoder.jp/contests/abc255/tasks/abc255_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        R: usize,
        C: usize,
        A: [[usize; 2]; 2],
    }
    println!("{}", A[R-1][C-1]);
}