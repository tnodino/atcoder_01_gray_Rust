// https://atcoder.jp/contests/abc189/tasks/abc189_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut X: usize,
    }
    X *= 100;
    let mut cnt = 0;
    for i in 1..=N {
        input! {
            V: usize,
            P: usize,
        }
        cnt += V * P;
        if cnt > X {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}