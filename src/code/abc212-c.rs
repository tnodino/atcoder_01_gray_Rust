// https://atcoder.jp/contests/abc212/tasks/abc212_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        mut A: [isize; N],
        mut B: [isize; M],
    }
    A.sort();
    B.sort();
    let mut bidx = 0;
    let mut ans = (A[0] - B[0]).abs();
    for i in 0..N {
        while bidx < M && A[i] >= B[bidx] {
            ans = min(ans, (A[i] - B[bidx]).abs());
            bidx += 1;
        }
        if bidx == M {
            break;
        }
        ans = min(ans, (A[i] - B[bidx]).abs());
    }
    println!("{}", ans);
}