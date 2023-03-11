// https://atcoder.jp/contests/abc179/tasks/abc179_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = 0;
    for a in 1..N {
        ans += (N - 1) / a;
    }
    println!("{}", ans);
}