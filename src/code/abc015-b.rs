// https://atcoder.jp/contests/abc015/tasks/abc015_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut bug = 0;
    let mut cnt = 0;
    for i in 0..N {
        if A[i] == 0 {
            continue;
        }
        bug += A[i];
        cnt += 1;
    }
    println!("{}", (bug + cnt - 1) / cnt);
}