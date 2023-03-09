// https://atcoder.jp/contests/abc156/tasks/abc156_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
        K: usize,
    }
    let mut ans = 0;
    while N > 0 {
        ans += 1;
        N /= K;
    }
    println!("{}", ans);
}