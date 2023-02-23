// https://atcoder.jp/contests/abc009/tasks/abc009_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }
    A.sort_by(|a, b| b.cmp(a));
    A.dedup();
    println!("{}", A[1]);
}