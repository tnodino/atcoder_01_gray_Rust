// https://atcoder.jp/contests/abc173/tasks/abc173_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
    }
    N %= 1000;
    println!("{}", (1000 - N) % 1000);
}