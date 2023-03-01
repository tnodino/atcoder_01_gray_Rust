// https://atcoder.jp/contests/abc277/tasks/abc277_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        P: [usize; N],
    }
    for i in 0..N {
        if P[i] == X {
            println!("{}", i + 1);
            return;
        }
    }
}