// https://atcoder.jp/contests/abc220/tasks/abc220_b

use proconio::input;
use proconio::fastout;

fn base_change(mut x: usize, base: usize) -> usize {
    let mut ret = 0;
    let mut d = 1;
    while x > 0 {
        ret += x % 10 * d;
        x /= 10;
        d *= base;
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
        A: usize,
        B: usize,
    }
    println!("{}", base_change(A, K) * base_change(B, K));
}