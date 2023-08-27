// https://atcoder.jp/contests/abc317/tasks/abc317_b

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
    for i in 0..N-1 {
        if A[i] + 1 != A[i+1] {
            println!("{}", A[i] + 1);
            return;
        }
    }
}