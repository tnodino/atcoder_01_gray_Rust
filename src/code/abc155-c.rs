// https://atcoder.jp/contests/abc155/tasks/abc155_c

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
        if !map.contains_key(&S) {
            map.insert(S.clone(), 0);
        }
        let v = map.get_mut(&S).unwrap();
        *v += 1;
    }
    let mut ans = Vec::new();
    let ma = map.values().max().unwrap();
    for (k, v) in map.iter() {
        if ma == v {
            ans.push(k);
        }
    }
    ans.sort();
    for a in ans {
        println!("{}", a);
    }
}