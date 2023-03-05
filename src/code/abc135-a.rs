// https://atcoder.jp/contests/abc135/tasks/abc135_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    if (A + B) % 2 == 0 {
        println!("{}", (A + B) / 2);
    }
    else {
        println!("IMPOSSIBLE");
    }
}