// https://atcoder.jp/contests/abc294/tasks/abc294_c

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; N],
        B: [usize; M],
    }
    let mut C = Vec::new();
    for a in A.iter() {
        C.push(a);
    }
    for b in B.iter() {
        C.push(b);
    }
    C.sort();
    let mut map = HashMap::new();
    for (i, c) in C.iter().enumerate() {
        map.insert(c, i + 1);
    }
    println!("{}", A.iter().map(|x| map[&x].to_string()).collect::<Vec<String>>().join(" "));
    println!("{}", B.iter().map(|x| map[&x].to_string()).collect::<Vec<String>>().join(" "));
}