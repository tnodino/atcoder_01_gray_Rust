// https://atcoder.jp/contests/abc052/tasks/abc052_b

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        S: String,
    }
    let mut x = 0;
    let mut ans = 0;
    for s in S.chars() {
        x += match s {
            'I' => 1,
            'D' => -1,
            _ => unreachable!()
        };
        ans = max(ans, x);
    }
    println!("{}", ans);
}