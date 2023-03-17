// https://atcoder.jp/contests/abc274/tasks/abc274_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut ans = vec![0; 2*N+1];
    for i in 0..N {
        ans[2*i+1] = ans[A[i]-1] + 1;
        ans[2*i+2] = ans[A[i]-1] + 1;
    }
    for i in 0..2*N+1 {
        println!("{}", ans[i]);
    }
}