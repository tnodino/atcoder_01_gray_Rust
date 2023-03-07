// https://atcoder.jp/contests/abc141/tasks/abc141_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: isize,
        Q: isize,
    }
    let mut cnt = vec![0; N];
    for _ in 0..Q {
        input! {
            A: usize,
        }
        cnt[A-1] += 1;
    }
    for i in 0..N {
        if K - (Q - cnt[i]) > 0 {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
}