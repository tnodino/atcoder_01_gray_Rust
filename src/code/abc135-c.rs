// https://atcoder.jp/contests/abc135/tasks/abc135_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N+1],
        B: [usize; N],
    }
    let mut ans = 0;
    for i in 0..N {
        if A[i] <= B[i] {
            ans += A[i];
            if A[i+1] <= B[i] - A[i] {
                ans += A[i+1];
                A[i+1] = 0;
            }
            else {
                let M = B[i] - A[i];
                ans += M;
                A[i+1] -= M;
            }
        }
        else {
            ans += B[i];
        }
    }
    println!("{}", ans);
}