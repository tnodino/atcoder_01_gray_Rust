// https://atcoder.jp/contests/abc296/tasks/abc296_c

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: isize,
        A: [isize; N],
    }
    let mut set = HashSet::new();
    for a in A.iter() {
        set.insert(a);
    }
    for a in A.iter() {
        if set.contains(&(a - X)) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}