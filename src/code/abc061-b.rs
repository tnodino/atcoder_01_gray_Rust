// https://atcoder.jp/contests/abc061/tasks/abc061_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut cnt = vec![0; N+1];
    for _ in 0..M {
        input! {
            a: usize,
            b: usize,
        }
        cnt[a] += 1;
        cnt[b] += 1;
    }
    for i in 1..=N {
        println!("{}", cnt[i]);
    }
}