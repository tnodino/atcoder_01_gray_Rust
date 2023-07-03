// https://atcoder.jp/contests/abc305/tasks/abc305_b

use proconio::input;
use proconio::fastout;
use std::mem::swap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        p: char,
        q: char,
    }
    let dist = [0, 3, 1, 4, 1, 5, 9];
    let mut p = (p as usize) - ('A' as usize);
    let mut q = (q as usize) - ('A' as usize);
    if p > q {
        swap(&mut p, &mut q);
    }
    let mut ans = 0;
    for i in p+1..=q {
        ans += dist[i];
    }
    println!("{}", ans);
}