// https://atcoder.jp/contests/abc182/tasks/abc182_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

fn popcnt(mut x: usize) -> usize {
    let mut ret = 0;
    while x > 0 {
        ret += x & 1;
        x >>= 1;
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
    }
    let mut vec = Vec::new();
    while N > 0 {
        vec.push(N % 10);
        N /= 10;
    }
    let M = vec.len();
    let mut ans = M;
    for bit in 1..1<<M {
        let mut x = 0;
        for i in 0..M {
            if bit & (1 << i) > 0 {
                x *= 10;
                x += vec[i];
            }
        }
        if x % 3 == 0 {
            ans = min(ans, M - popcnt(bit));
        }
    }
    if ans == M {
        println!("-1");
    }
    else {
        println!("{}", ans);
    }
}