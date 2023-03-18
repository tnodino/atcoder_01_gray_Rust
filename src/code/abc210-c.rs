// https://atcoder.jp/contests/abc210/tasks/abc210_c

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        c: [usize; N],
    }
    let mut map = HashMap::new();
    for i in 0..K {
        *map.entry(&c[i]).or_insert(0) += 1;
    }
    let mut cnt = map.len();
    let mut ans = cnt;
    for i in K..N {
        *map.entry(&c[i]).or_insert(0) += 1;
        if map[&c[i]] == 1 {
            cnt += 1;
        }
        *map.entry(&c[i-K]).or_insert(0) -= 1;
        if map[&c[i-K]] == 0 {
            cnt -= 1;
        }
        ans = max(ans, cnt);
    }
    println!("{}", ans);
}