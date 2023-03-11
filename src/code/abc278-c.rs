// https://atcoder.jp/contests/abc278/tasks/abc278_c

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        Q: usize,
    }
    let mut set = HashSet::new();
    for _ in 0..Q {
        input! {
            T: usize,
            A: usize,
            B: usize,
        }
        if T == 1 {
            set.insert((A, B));
        }
        else if T == 2 {
            set.remove(&(A, B));
        }
        else {
            if set.contains(&(A, B)) && set.contains(&(B, A)) {
                println!("Yes");
            }
            else {
                println!("No");
            }
        }
    }
}