// https://atcoder.jp/contests/abc317/tasks/abc317_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        H: usize,
        X: usize,
        P: [usize; N],
    }
    for i in 0..N {
        if X <= H + P[i] {
            println!("{}", i+1);
            return;
        }
    }
}