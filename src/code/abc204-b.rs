// https://atcoder.jp/contests/abc204/tasks/abc204_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut ans = 0;
    for a in A {
        if a > 10 {
            ans += a - 10;
        }
    }
    println!("{}", ans);
}