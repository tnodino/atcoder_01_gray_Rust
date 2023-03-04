// https://atcoder.jp/contests/aising2019/tasks/aising2019_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
        P: [usize; N],
    }
    let mut cnt = [0, 0, 0];
    for p in P.iter() {
        if p <= &A {
            cnt[0] += 1;
        }
        else if p <= &B {
            cnt[1] += 1;
        }
        else {
            cnt[2] += 1;
        }
    }
    println!("{}", cnt.iter().min().unwrap());
}