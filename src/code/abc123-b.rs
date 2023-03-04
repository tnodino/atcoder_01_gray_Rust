// https://atcoder.jp/contests/abc123/tasks/abc123_b

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
        D: usize,
        E: usize,
    }
    let array = [A, B, C, D, E];
    let mut time = 10;
    for i in 0..5 {
        if array[i] % 10 == 0 {
            continue;
        }
        time = min(time, array[i] % 10);
    }
    let mut ans = 0;
    for i in 0..5 {
        if array[i] % 10 == 0 || array[i] % 10 == time {
            ans += array[i];
            if array[i] % 10 == time {
                time = 10;
            }
        }
        else {
            ans += (array[i] + 9) / 10 * 10;
        }
    }
    println!("{}", ans);
}