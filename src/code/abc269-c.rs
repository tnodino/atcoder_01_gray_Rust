// https://atcoder.jp/contests/abc269/tasks/abc269_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut vec = Vec::new();
    for i in 0..60 {
        if N & (1 << i) > 0 {
            vec.push(i);
        }
    }
    let M = vec.len();
    for bit in 0..1<<M {
        let mut ans: usize = 0;
        for i in 0..M {
            if bit & (1 << i) > 0 {
                ans += 1 << vec[i];
            }
        }
        println!("{}", ans);
    }
}