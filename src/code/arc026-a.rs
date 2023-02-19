// https://atcoder.jp/contests/arc026/tasks/arc026_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
    }
    let ans;
    if N <= 5 {
        ans = N * B;
    }
    else {
        ans = A * (N - 5) + B * 5;
    }
    println!("{}", ans);
}