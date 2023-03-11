// https://atcoder.jp/contests/abc176/tasks/abc176_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }
    let mut ans = 0;
    for i in 1..N {
        if A[i-1] > A[i] {
            ans += A[i-1] - A[i];
            A[i] = A[i-1];
        }
    }
    println!("{}", ans);
}