// https://atcoder.jp/contests/abc164/tasks/abc164_c

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
        set.insert(S);
    }
    println!("{}", set.len());
}