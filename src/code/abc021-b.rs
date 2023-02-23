// https://atcoder.jp/contests/abc021/tasks/abc021_b

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: usize,
        b: usize,
        K: usize,
        P: [usize; K],
    }
    let mut S = HashSet::new();
    S.insert(a);
    S.insert(b);
    for p in P {
        if S.contains(&p) {
            println!("NO");
            return;
        }
        S.insert(p);
    }
    println!("YES");
}