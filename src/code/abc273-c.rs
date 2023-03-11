// https://atcoder.jp/contests/abc273/tasks/abc273_c

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut map = HashMap::new();
    for a in A.iter() {
        *map.entry(a).or_insert(0) += 1;
    }
    let mut vec = map.iter().collect::<Vec<(_, _)>>();
    vec.sort_by(|a, b| b.0.cmp(a.0));
    for i in 0..N {
        if i >= vec.len() {
            println!("0");
        }
        else {
            println!("{}", vec[i].1);
        }
    }
}