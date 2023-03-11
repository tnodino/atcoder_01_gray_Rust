// https://atcoder.jp/contests/abc193/tasks/abc193_c

use proconio::input;
use proconio::fastout;
use num::pow;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut set = HashSet::new();
    for a in 2..=N {
        if pow(a, 2) > N {
            break;
        }
        for b in 2..=N {
            if pow(a, b) > N {
                break;
            }
            set.insert(pow(a, b));
        }
    }
    println!("{}", N - set.len());
}