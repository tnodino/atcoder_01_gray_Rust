// https://atcoder.jp/contests/abc290/tasks/abc290_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        mut A: [usize; N],
    }
    A.sort();
    A.dedup();
    A.push(!0 as usize);
    for i in 0..K {
        if A[i] == !0 as usize || A[i] != i {
            println!("{}", i);
            return;
        }
    }
    println!("{}", K);
}