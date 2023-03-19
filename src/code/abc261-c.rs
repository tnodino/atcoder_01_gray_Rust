// https://atcoder.jp/contests/abc261/tasks/abc261_c

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut map = HashMap::new();
    for _ in 0..N {
        input! {
            S: String,
        }
        if map.contains_key(&S) {
            println!("{}({})", S, map[&S]);
        }
        else {
            println!("{}", S);
        }
        *map.entry(S).or_insert(0) += 1;
    }
}