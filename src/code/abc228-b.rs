// https://atcoder.jp/contests/abc228/tasks/abc228_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        A: [usize; N],
    }
    let mut flg = vec![false; N];
    let mut idx = X - 1;
    while !flg[idx] {
        flg[idx] = true;
        idx = A[idx] - 1;
    }
    println!("{}", flg.iter().filter(|&x| *x).count());
}