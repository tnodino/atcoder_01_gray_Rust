// https://atcoder.jp/contests/abc173/tasks/abc173_b

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
    let array = ["AC", "WA", "TLE", "RE"].iter().map(|&x| x.to_string()).collect::<Vec<String>>();
    for i in 0..4 {
        map.insert(&array[i], 0);
    }
    for _ in 0..N {
        input! {
            S: String,
        }
        let v = map.get_mut(&S).unwrap();
        *v += 1;
    }
    for i in 0..4 {
        println!("{} x {}", array[i], map.get(&array[i]).unwrap());
    }
}