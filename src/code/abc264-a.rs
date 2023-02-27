// https://atcoder.jp/contests/abc264/tasks/abc264_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        L: usize,
        R: usize,
    }
    println!("{}", &"atcoder"[L-1..R]);
}