// https://atcoder.jp/contests/abc180/tasks/abc180_c

use proconio::input;
use proconio::fastout;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = sqrt(N as f64) as usize;
    let mut ans = Vec::new();
    for i in 1..=M {
        if N % i == 0 {
            ans.push(i);
            ans.push(N/i);
        }
    }
    ans.sort();
    ans.dedup();
    for a in ans {
        println!("{}", a);
    }
}