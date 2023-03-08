// https://atcoder.jp/contests/abc245/tasks/abc245_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }
    A.sort();
    A.dedup();
    for i in 0..A.len() {
        if A[i] != i {
            println!("{}", i);
            return;
        }
    }
    println!("{}", A.len());
}