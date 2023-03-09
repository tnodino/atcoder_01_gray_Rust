// https://atcoder.jp/contests/abc241/tasks/abc241_b

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
    let mut map = HashMap::new();
    for a in A.iter() {
        if !map.contains_key(a) {
            map.insert(a, 0);
        }
        let v = map.get_mut(a).unwrap();
        *v += 1;
    }
    for b in B.iter() {
        if !map.contains_key(b) {
            map.insert(b, 0);
        }
        if map[b] == 0 {
            println!("No");
            return;
        }
        let v = map.get_mut(b).unwrap();
        *v -= 1;
    }
    println!("Yes");
}