// https://atcoder.jp/contests/abc220/tasks/abc220_b

use proconio::input;
use proconio::fastout;

fn f(mut x: usize, k: usize) -> usize {
    let mut ret = 0;
    let mut p = 1;
    while x > 0 {
        ret += x % 10 * p;
        x /= 10;
        p *= k;
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (K, A, B): (usize, usize, usize),
    }
    println!("{}", f(A, K) * f(B, K));
}