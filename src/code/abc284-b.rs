// https://atcoder.jp/contests/abc284/tasks/abc284_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn solve() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut ans = 0;
    for i in 0..N {
        if A[i] % 2 == 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}


#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
    }
    for _ in 0..T {
        solve();
    }
}