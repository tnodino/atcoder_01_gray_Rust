// https://atcoder.jp/contests/abc101/tasks/abc101_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut ans = 0;
    for s in S.chars() {
        match s {
            '+' => ans += 1,
            _ => ans -= 1,
        }
    }
    println!("{}", ans);
}