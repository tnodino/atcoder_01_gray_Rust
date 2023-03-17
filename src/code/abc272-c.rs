// https://atcoder.jp/contests/abc272/tasks/abc272_c

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [isize; N],
    }
    let mut odd = Vec::new();
    let mut even = Vec::new();
    for a in A {
        match a % 2 {
            0 => odd.push(a),
            _ => even.push(a),
        }
    }
    odd.sort_by(|a, b| b.cmp(a));
    even.sort_by(|a, b| b.cmp(a));
    let mut ans = -1;
    if odd.len() >= 2 {
        ans = max(ans, odd[0] + odd[1]);
    }
    if even.len() >= 2 {
        ans = max(ans, even[0] + even[1]);
    }
    println!("{}", ans);
}