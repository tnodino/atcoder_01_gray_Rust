// https://atcoder.jp/contests/abc181/tasks/abc181_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = 0;
    for _ in 0..N {
        input! {
            A: usize,
            B: usize,
        }
        ans += (B + 1) * B / 2 - (A - 1) * A / 2;
    }
    println!("{}", ans);
}