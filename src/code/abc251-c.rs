// https://atcoder.jp/contests/abc251/tasks/abc251_c

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut set = HashSet::new();
    let mut ma = 0;
    let mut ans = 0;
    for i in 0..N {
        input! {
            S: String,
            T: usize,
        }
        if set.contains(&S) {
            continue;
        }
        set.insert(S);
        if ma < T {
            ma = T;
            ans = i + 1;
        }
    }
    println!("{}", ans);
}