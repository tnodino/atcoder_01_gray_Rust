// https://atcoder.jp/contests/arc025/tasks/arc025_1

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let N = 7;
    input! {
        D: [usize; N],
        J: [usize; N],
    }
    let mut ans = 0;
    for i in 0..N {
        ans += max(D[i], J[i]);
    }
    println!("{}", ans);
}