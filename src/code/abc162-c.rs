// https://atcoder.jp/contests/abc162/tasks/abc162_c

use proconio::input;
use proconio::fastout;
use num::integer::gcd;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
    }
    let mut ans = 0;
    for a in 1..=K {
        for b in 1..=K {
            for c in 1..=K {
                ans += gcd(a, gcd(b, c));
            }
        }
    }
    println!("{}", ans);
}