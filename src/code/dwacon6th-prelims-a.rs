// https://atcoder.jp/contests/dwacon6th-prelims/tasks/dwacon6th_prelims_a

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
    let mut s = Vec::new();
    let mut t = Vec::new();
    for _ in 0..N {
        input! {
            a: String,
            b: usize,
        }
        s.push(a);
        t.push(b);
    }
    let mut cnt = 0;
    for i in (0..N).rev() {
        map.insert(&s[i], cnt);
        cnt += t[i];
    }
    input! {
        X: String,
    }
    println!("{}", map.get(&X).unwrap());
}