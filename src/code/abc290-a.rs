// https://atcoder.jp/contests/abc290/tasks/abc290_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; N],
        B: [usize; M],
    }
    let mut ans = 0;
    for i in 0..M {
        ans += A[B[i]-1];
    }
    println!("{}", ans);
}