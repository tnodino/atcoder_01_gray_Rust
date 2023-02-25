// https://atcoder.jp/contests/abc031/tasks/abc031_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        L: isize,
        H: isize,
        N: usize,
    }
    for _ in 0..N {
        input! {
            A: isize,
        }
        if A < L {
            println!("{}", L - A);
        }
        else if H < A {
            println!("-1");
        }
        else {
            println!("0");
        }
    }
}