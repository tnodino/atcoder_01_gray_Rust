// https://atcoder.jp/contests/diverta2019/tasks/diverta2019_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    println!("{}", N - K + 1);
}