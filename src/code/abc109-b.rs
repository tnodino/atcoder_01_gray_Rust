// https://atcoder.jp/contests/abc109/tasks/abc109_b

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut W: String,
    }
    let mut set = HashSet::new();
    set.insert(W.clone());
    for _ in 0..N-1 {
        input! {
            W2: String,
        }
        if W.chars().last().unwrap() != W2.chars().next().unwrap() {
            println!("No");
            return;
        }
        if set.contains(&W2) {
            println!("No");
            return;
        }
        set.insert(W2.clone());
        W = W2;
    }
    println!("Yes");
}