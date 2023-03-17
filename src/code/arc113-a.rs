// https://atcoder.jp/contests/arc113/tasks/arc113_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
    }
    let mut ans = 0;
    for a in 1..=K {
        for b in 1..=K {
            if a * b > K {
                break;
            }
            let x = a * b;
            ans += K / x;
        }
    }
    println!("{}", ans);
}