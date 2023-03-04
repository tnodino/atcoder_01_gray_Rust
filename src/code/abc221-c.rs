// https://atcoder.jp/contests/abc221/tasks/abc221_c

use proconio::input;
use proconio::fastout;
use itertools::Itertools;
use std::cmp::max;

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
    let mut ans = 0;
    for perm in (0..M).permutations(M) {
        for j in 1..M {
            let mut l = 0;
            let mut r = 0;
            for k in 0..j {
                l *= 10;
                l += vec[perm[k]];
            }
            for k in j..M {
                r *= 10;
                r += vec[perm[k]];
            }
            ans = max(ans, l * r);
        }
    }
    println!("{}", ans);
}