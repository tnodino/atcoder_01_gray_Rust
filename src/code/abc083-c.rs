// https://atcoder.jp/contests/abc083/tasks/arc088_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        Y: usize,
    }
    let mut x = X;
    let mut ans = 1;
    while x * 2 <= Y {
        x *= 2;
        ans += 1;
    }
    println!("{}", ans);
}