// https://atcoder.jp/contests/abc226/tasks/abc226_b

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
            L: usize,
            a: [usize; L],
        }
        set.insert(a);
    }
    println!("{}", set.len());
}