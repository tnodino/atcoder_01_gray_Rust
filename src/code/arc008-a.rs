// https://atcoder.jp/contests/arc008/tasks/arc008_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let A = (N / 10) * 100 + (N % 10) * 15;
    let B = ((N + 9) / 10) * 100;
    println!("{}", A.min(B));
}