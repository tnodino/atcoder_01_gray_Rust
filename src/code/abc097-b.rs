// https://atcoder.jp/contests/abc097/tasks/abc097_b

use proconio::input;
use proconio::fastout;
use num::pow;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
    }
    let mut ans = 1;
    for b in 1..=X {
        for p in 2..=X {
            let po = pow(b, p);
            if X < po {
                break;
            }
            ans = max(ans, po);
        }
    }
    println!("{}", ans);
}