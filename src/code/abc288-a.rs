// https://atcoder.jp/contests/abc288/tasks/abc288_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    for _ in 0..N {
        input! {
            A: isize,
            B: isize,
        }
        println!("{}", A + B)
    }
}