// https://atcoder.jp/contests/abc206/tasks/abc206_c

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
    for i in 0..N {
        *map.entry(A[i]).or_insert(0) += 1;
    }
    let mut ans = 0;
    for v in map.values() {
        ans += v * (N - v);
    }
    println!("{}", ans / 2);
}