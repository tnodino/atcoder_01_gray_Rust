// https://atcoder.jp/contests/arc016/tasks/arc016_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    println!("{}", match M {
        1 => N,
        _ => 1,
    });
}