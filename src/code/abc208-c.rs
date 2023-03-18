// https://atcoder.jp/contests/abc208/tasks/abc208_c

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        a: [usize; N],
    }
    let mut b = a.clone();
    b.sort();
    let mut map = HashMap::new();
    for i in 0..N {
        if i < K % N {
            map.insert(b[i], K / N + 1);
        }
        else {
            map.insert(b[i], K / N);
        }
    }
    for i in 0..N {
        println!("{}", map.get(&a[i]).unwrap());
    }
}