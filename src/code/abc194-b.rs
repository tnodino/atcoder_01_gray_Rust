// https://atcoder.jp/contests/abc194/tasks/abc194_b

use proconio::input;
use proconio::fastout;
use std::cmp::max;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut A = Vec::new();
    let mut B = Vec::new();
    for _ in 0..N {
        input! {
            a: usize,
            b: usize,
        }
        A.push(a);
        B.push(b);
    }
    let mut ans: usize = !0;
    for i in 0..N {
        for j in 0..N {
            if i == j {
                ans = min(ans, A[i] + B[i]);
            }
            else {
                ans = min(ans, max(A[i], B[j]));
            }
        }
    }
    println!("{}", ans);
}