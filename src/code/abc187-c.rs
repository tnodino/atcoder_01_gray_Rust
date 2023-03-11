// https://atcoder.jp/contests/abc187/tasks/abc187_c

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut S = Vec::new();
    let mut set = HashSet::new();
    for _ in 0..N {
        input! {
            s: String,
        }
        S.push(s.clone());
        set.insert(s.clone());
    }
    for s in S {
        if set.contains(&format!("{}{}", "!", s)) {
            println!("{}", s);
            return;
        }
    }
    println!("satisfiable");
}