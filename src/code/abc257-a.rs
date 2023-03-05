// https://atcoder.jp/contests/abc257/tasks/abc257_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
    }
    for i in 1..=26 {
        if N * i >= X {
            println!("{}", (i as u8 + 64) as char);
            return;
        }
    }
}