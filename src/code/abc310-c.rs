// https://atcoder.jp/contests/abc310/tasks/abc310_c

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
    for _ in 0..N {
        input! {
            S: String,
        }
        let T = S.chars().rev().collect::<String>();
        if set.contains(&S) || set.contains(&T) {
            continue;
        }
        set.insert(S);
    }
    println!("{}", set.len());
}