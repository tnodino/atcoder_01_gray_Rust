// https://atcoder.jp/contests/abc140/tasks/abc140_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        B: [usize; N],
        C: [usize; N-1],
    }
    let mut ans = 0;
    let mut before = 99;
    for i in 0..N {
        ans += B[A[i]-1];
        if before + 1 == A[i] {
            ans += C[before-1];
        }
        before = A[i];
    }
    println!("{}", ans);
}