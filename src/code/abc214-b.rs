// https://atcoder.jp/contests/abc214/tasks/abc214_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: usize,
        T: usize,
    }
    let mut ans = 0;
    for a in 0..=S {
        for b in 0..=S {
            for c in 0..=S {
                if a + b + c <= S && a * b * c <= T {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}