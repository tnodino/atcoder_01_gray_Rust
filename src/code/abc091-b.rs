// https://atcoder.jp/contests/abc091/tasks/abc091_b

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut map = HashMap::new();
    for _ in 0..N {
        input! {
            s: String,
        }
        if map.contains_key(&s) {
            let v = map.get_mut(&s).unwrap();
            *v += 1;
        }
        else {
            map.insert(s, 1);
        }
    }
    input! {
        M: usize,
    }
    for _ in 0..M {
        input! {
            s: String,
        }
        if map.contains_key(&s) {
            let v = map.get_mut(&s).unwrap();
            *v -= 1;
        }
        else {
            map.insert(s, -1);
        }
    }
    let mut ans = 0;
    for (_, cnt) in map.iter() {
        ans = max(ans, *cnt);
    }
    println!("{}", ans);
}