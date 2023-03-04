// https://atcoder.jp/contests/aising2019/tasks/aising2019_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        H: usize,
        W: usize,
    }
    println!("{}", (N - H + 1) * (N - W + 1));
}