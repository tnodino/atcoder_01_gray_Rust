// https://atcoder.jp/contests/abc297/tasks/abc297_b

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut map = HashMap::new();
    for (i, s) in S.chars().enumerate() {
        map.entry(s).or_insert(Vec::new()).push(i);
    }
    let B = map.get(&'B').unwrap();
    let K = map.get(&'K').unwrap();
    let R = map.get(&'R').unwrap();
    if B[0] % 2 == B[1] % 2 {
        println!("No");
    }
    else if K[0] < R[0] || R[1] < K[0] {
        println!("No");
    }
    else {
        println!("Yes");
    }
}