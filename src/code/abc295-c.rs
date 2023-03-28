// https://atcoder.jp/contests/abc295/tasks/abc295_c

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut map = HashMap::new();
    for a in A {
        *map.entry(a).or_insert(0) += 1;
    }
    let mut ans = 0;
    for v in map.values() {
        ans += v / 2;
    }
    println!("{}", ans);
}