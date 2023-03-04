// https://atcoder.jp/contests/abc120/tasks/abc120_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        K: usize,
    }
    let mut cnt = 0;
    for i in (1..=A).rev() {
        if A % i == 0 && B % i == 0 {
            cnt += 1;
        }
        if cnt == K {
            println!("{}", i);
            return;
        }
    }
}