// https://atcoder.jp/contests/arc009/tasks/arc009_1

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
            a: usize,
            b: usize,
        }
        ans += a * b;
    }
    ans *= 105;
    ans /= 100;
    println!("{}", ans);
}