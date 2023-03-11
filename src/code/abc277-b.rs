// https://atcoder.jp/contests/abc277/tasks/abc277_b

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
    let mark = "HDCS".chars().collect::<Vec<char>>();
    let num = "A23456789TJQK".chars().collect::<Vec<char>>();
    for _ in 0..N {
        input! {
            S: String,
        }
        let S = S.chars().collect::<Vec<char>>();
        if !mark.contains(&S[0]) || !num.contains(&S[1]) || set.contains(&S) {
            println!("No");
            return;
        }
        set.insert(S);
    }
    println!("Yes");
}