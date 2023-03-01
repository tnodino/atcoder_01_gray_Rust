// https://atcoder.jp/contests/abc096/tasks/abc096_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
        K: usize,
    }
    let mut array = [A, B, C];
    array.sort_by(|a, b| b.cmp(a));
    for _ in 0..K {
        array[0] *= 2;
    }
    println!("{}", array.iter().sum::<usize>());
}