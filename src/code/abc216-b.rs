// https://atcoder.jp/contests/abc216/tasks/abc216_b

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
            T: String,
        }
        if set.contains(&(S.clone(), T.clone())) {
            println!("Yes");
            return;
        }
        set.insert((S, T));
    }
    println!("No");
}