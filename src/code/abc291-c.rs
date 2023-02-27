// https://atcoder.jp/contests/abc291/tasks/abc291_c

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        S: String,
    }
    let mut map = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    map.insert((x, y));
    for s in S.chars() {
        match s {
            'R' => x += 1,
            'L' => x -= 1,
            'U' => y += 1,
            'D' => y -= 1,
            _ => unreachable!()
        }
        if map.contains(&(x, y)) {
            println!("Yes");
            return;
        }
        map.insert((x, y));
    }
    println!("No");
}