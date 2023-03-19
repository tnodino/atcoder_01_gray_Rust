// https://atcoder.jp/contests/abc215/tasks/abc215_c

use proconio::input;
use proconio::fastout;
use itertools::Itertools;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        mut K: usize,
    }
    let N = S.len();
    let mut S = S.chars().collect::<Vec<char>>();
    S.sort();
    let mut set = HashSet::new();
    for perm in (0..N).permutations(N) {
        let mut s = "".to_string();
        for i in 0..N {
            s = format!("{}{}", s, S[perm[i]]);
        }
        if !set.contains(&s) {
            K -= 1;
            if K == 0 {
                println!("{}", s);
                return;
            }
            set.insert(s);
        }
    }
}