// https://atcoder.jp/contests/abc207/tasks/abc207_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
    }
    let mut array = [A, B, C];
    array.sort_by(|a, b| b.cmp(a));
    println!("{}", array[0] + array[1]);
}