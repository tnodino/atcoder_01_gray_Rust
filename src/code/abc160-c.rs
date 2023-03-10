// https://atcoder.jp/contests/abc160/tasks/abc160_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
        N: usize,
        mut A: [usize; N],
    }
    A = A.repeat(2);
    for i in N..N*2 {
        A[i] += K;
    }
    let mut ans: usize = !0;
    for i in 0..N {
        ans = min(ans, A[i+N-1] - A[i]);
    }
    println!("{}", ans);
}