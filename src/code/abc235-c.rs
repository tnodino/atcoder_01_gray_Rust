// https://atcoder.jp/contests/abc235/tasks/abc235_c

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
        a: [usize; N],
    }
    let mut map = HashMap::new();
    for i in 0..N {
        map.entry(a[i]).or_insert(Vec::new()).push(i + 1);
    }
    for _ in 0..Q {
        input! {
            x: usize,
            k: usize,
        }
        if map.contains_key(&x) && map[&x].len() >= k {
            println!("{}", map[&x][k-1]);
        }
        else {
            println!("-1");
        }
    }
}