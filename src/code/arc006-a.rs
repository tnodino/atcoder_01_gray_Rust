// https://atcoder.jp/contests/arc006/tasks/arc006_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        E: [usize; 6],
        B: usize,
        L: [usize; 6],
    }
    let mut hit = 0;
    let mut bonus = false;
    for i in 0..6 {
        if L.contains(&E[i]) {
            hit += 1;
        }
    }
    if L.contains(&B) {
        bonus = true;
    }
    let ans;
    match hit {
        6 => ans = 1,
        5 if bonus => ans = 2,
        5 => ans = 3,
        4 => ans = 4,
        3 => ans = 5,
        _ => ans = 0,
    }
    println!("{}", ans);
}