// https://atcoder.jp/contests/jsc2021/tasks/jsc2021_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    for i in (1..=B).rev() {
        if (A + i - 1) / i < B / i {
            println!("{}", i);
            return;
        }
    }
}