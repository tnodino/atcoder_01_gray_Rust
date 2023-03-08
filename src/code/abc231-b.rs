// https://atcoder.jp/contests/abc231/tasks/abc231_b

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
    let mut ans = "";
    let mut cnt = 0;
    for (k, v) in map.iter() {
        if *v > cnt {
            ans = k;
            cnt = *v;
        }
    }
    println!("{}", ans);
}