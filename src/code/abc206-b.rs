// https://atcoder.jp/contests/abc206/tasks/abc206_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut S = 0;
    for i in 1..=N {
        S += i;
        if S >= N {
            println!("{}", i);
            return;
        }
    }
}