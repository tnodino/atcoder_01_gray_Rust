// https://atcoder.jp/contests/abc300/tasks/abc300_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
        C: [usize; N],
    }
    for i in 0..N {
        if A + B == C[i] {
            println!("{}", i + 1);
            return;
        }
    }
}